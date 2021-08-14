mod pages;
mod pinball;

use pages::home::Home;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <ion-app>
            <ion-split-pane content-id="main">
                <ion-menu content-id="main">
                    <ion-header>
                        <ion-toolbar>
                            <ion-title>{"Menu"}</ion-title>
                        </ion-toolbar>
                    </ion-header>
                </ion-menu>
                <div id="main" class="ion-page">
                    <Router<Route> render={Router::render(switch)} />
                </div>
            </ion-split-pane>
        </ion-app>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        _ => {
            html! { <Home />}
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    yew::start_app::<App>();
}
