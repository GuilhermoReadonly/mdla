use mdla_lib::model::Validation;

use yew::prelude::*;

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
