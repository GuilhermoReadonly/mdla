use log::info;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement, HtmlInputElement};
use yew::prelude::*;

#[derive(Debug)]
pub struct GridInputComponent {
    pub guessed_word: String,
}

#[derive(Debug, Properties, PartialEq)]
pub struct GridInputProperties {
    pub width: usize,
    pub on_guessed_word_change: Callback<String>,
    pub on_validate: Callback<()>,
}

#[derive(Debug)]
pub enum Msg {
    UpdateGuess(String, usize),
    Validate,
    NoValidate,
}

impl GridInputComponent {
    fn focus_on_id(&self, id: &str) {
        info!("Focus on {id}");
        let elt = window()
            .expect("no global `window` exists")
            .document()
            .expect("should have a document on window")
            .get_element_by_id(id);

        if let Some(e) = elt {
            e.dyn_ref::<HtmlElement>()
                .expect("#id should be an `HtmlElement`")
                .focus()
                .expect("focus should be ok on input element")
        };
    }

    fn reset(&self, id: &str) {
        info!("Reset input {id}");
        let elt = window()
            .expect("no global `window` exists")
            .document()
            .expect("should have a document on window")
            .get_element_by_id(id);

        info!("Reset input {elt:?}");
        if let Some(e) = elt {
            e.dyn_ref::<HtmlInputElement>()
                .expect("#id should be an `HtmlElement`")
                .set_value("")
        };
    }
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

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let id = "input-cell-0";
            self.focus_on_id(id);
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cells = 0..ctx.props().width;

        cells
            .into_iter()
            .map(|cell_number| {
                let oninput = {
                    ctx.link().callback(move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        let value = input.value();
                        Msg::UpdateGuess(value, cell_number)
                    })
                };
                let onkeypress = {
                    ctx.link().callback(move |e: KeyboardEvent| {
                        info!("Keyboard event: {e:?}");
                        if &e.code() == "Enter" {
                            Msg::Validate
                        } else {
                            Msg::NoValidate
                        }
                    })
                };

                let id = format!("input-cell-{cell_number}");

                html! {
                    <>
                        <td>
                            <input
                                {id}
                                class="input-cell"
                                type="text"
                                maxlength="1"
                                {oninput}
                                {onkeypress}
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
                match (char.is_empty(), position < self.guessed_word.len()) {
                    (false, true) => {
                        self.guessed_word.insert_str(position, &char);
                        let id = format!("input-cell-{}", position + 1);
                        self.focus_on_id(id.as_str())
                    }
                    (false, false) => {
                        self.guessed_word.push_str(&char);
                        let id = format!("input-cell-{}", position + 1);
                        self.focus_on_id(id.as_str())
                    }
                    (true, true) => {
                        self.guessed_word.remove(position);
                    }
                    (true, false) => {
                        self.guessed_word.pop();
                    }
                }

                ctx.props()
                    .on_guessed_word_change
                    .emit(self.guessed_word.clone());
            }
            Msg::Validate => {
                self.guessed_word = String::new();
                for i in 0..ctx.props().width {
                    let id = format!("input-cell-{i}");
                    self.reset(&id);
                }
                self.focus_on_id("input-cell-0");
                ctx.props().on_validate.emit(());
            }
            Msg::NoValidate => {}
        }
        true
    }
}
