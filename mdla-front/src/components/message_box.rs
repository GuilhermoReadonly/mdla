use yew::prelude::*;

#[derive(Debug)]
pub enum Msg {}

#[derive(Debug)]
pub struct Message {
    text: String,
    severity: String,
}

#[derive(Debug)]
pub struct MessageBox {
    message: Option<Message>,
}

impl Component for MessageBox {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { message: None }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match &self.message {
            None => html!(),
            Some(m) => html!(<>{&m.severity} {":"} {&m.text}</>),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        log::info!("Message received: {:?}", msg);
        true
    }
}
