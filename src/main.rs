use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
use pages::home::Home;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/posts/:id")]
    Post { id: u64 },
    #[at("/posts")]
    Posts,
    #[at("/authors/:id")]
    Author { id: u64 },
    #[at("/authors")]
    Authors,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
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
    yew::start_app::<App>();
}
