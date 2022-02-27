use crate::components::page_home::HomePageComponent;

use log::info;
use yew::prelude::*;
use yew_router::prelude::*;

mod page_home;

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Index,
}

#[derive(Debug)]
pub(crate) struct MainComponent {}

impl Component for MainComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="grid-container">
                <BrowserRouter>
                    <div class="content">
                        <Switch<AppRoute> render={Switch::render(move |routes: &AppRoute| {
                            info!("Route: {:?}", routes);
                            match routes.clone() {
                                AppRoute::Index => html!{<HomePageComponent/>},
                            }
                        })} />
                    </div>
                </BrowserRouter>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }
}
