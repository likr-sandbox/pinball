mod pages;
mod pinball;

use pages::game::GamePage;
use pages::home::HomePage;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/game")]
    Game,
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <ion-app>
            <ion-split-pane content-id="main">
                <ion-menu content-id="main">
                    <ion-header>
                        <ion-toolbar>
                            <ion-title>{"スマボ de 盆"}</ion-title>
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
        Route::Home => {
            html! { <HomePage />}
        }
        Route::Game => {
            html! { <GamePage />}
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    yew::start_app::<App>();
}
