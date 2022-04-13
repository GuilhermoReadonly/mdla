use yew::prelude::*;

use super::page_game::Message;

#[derive(Debug, Properties, PartialEq)]
pub struct MessageBoxProperties {
    pub message: Option<Message>,
}

#[derive(Debug)]
pub struct MessageBox;

impl Component for MessageBox {
    type Message = ();
    type Properties = MessageBoxProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &ctx.props().message {
            None => html!(),
            Some(m) => html!(<>{&m.text}</>),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: ()) -> bool {
        false
    }
}
