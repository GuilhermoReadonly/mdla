use log::info;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{window, HtmlElement, HtmlInputElement};
use yew::prelude::*;

#[derive(Debug)]
pub struct GridInputComponent {
    pub guessed_word: String,
    // We need this dummy field just to store the Closure...
    // Otherwise the closure is destroyed when going out of scope.
    body_on_click: Closure<dyn Fn()>,
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

impl Component for GridInputComponent {
    type Message = Msg;
    type Properties = GridInputProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        let body_on_click = Closure::wrap(Box::new(move || {
            info!("Click on body");

            if !is_focus_on_input() {
                focus_on_id("input-cell-0");
            }
        }) as Box<dyn Fn()>);

        Self {
            guessed_word: String::default(),
            body_on_click,
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        info!("Rendered");
        if first_render {
            let body = window()
                .expect("should have a window in this context")
                .document()
                .expect("window should have a document")
                .body()
                .expect("A body should be there");

            body.add_event_listener_with_callback(
                "click",
                self.body_on_click.as_ref().unchecked_ref(),
            )
            .expect("On click call back should work");

            focus_on_id("input-cell-0");
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
                        info!("Keyboard event: {}", e.code());
                        if &e.code() == "Enter" {
                            Msg::Validate
                        } else if &e.code() == "Return" {
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
                        focus_on_id(id.as_str())
                    }
                    (false, false) => {
                        self.guessed_word.push_str(&char);
                        let id = format!("input-cell-{}", position + 1);
                        focus_on_id(id.as_str())
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
                reset_all_inputs(ctx.props().width);

                focus_on_id("input-cell-0");
                ctx.props().on_validate.emit(());
            }
            Msg::NoValidate => {}
        }
        true
    }
}

fn focus_on_id(id: &str) {
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

fn is_focus_on_input() -> bool {
    let elt = window()
        .expect("no global `window` exists")
        .document()
        .expect("should have a document on window")
        .active_element();

    if let Some(elt) = elt {
        elt.id().contains("input-cell")
    } else {
        false
    }
}

fn reset_all_inputs(n: usize) {
    info!("Reset all input");

    for i in 0..n {
        let id = format!("input-cell-{i}");
        let elt = window()
            .expect("no global `window` exists")
            .document()
            .expect("should have a document on window")
            .get_element_by_id(&id);

        info!("Reset input {elt:?}");
        if let Some(e) = elt {
            e.dyn_ref::<HtmlInputElement>()
                .expect("#id should be an `HtmlElement`")
                .set_value("")
        };
    }
}
