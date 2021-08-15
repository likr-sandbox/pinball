use crate::Route;
use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let handle_click = Callback::from(move |_| {
        yew_router::push_route(Route::Game);
    });
    html! {
        <>
            <ion-header>
                <ion-toolbar>
                    <ion-buttons>
                        <ion-menu-button />
                    </ion-buttons>
                    <ion-title>{"ゲームセレクト"}</ion-title>
                </ion-toolbar>
            </ion-header>
            <ion-content fullscreen="fullscreen" class="ion-padding" scroll-y="false" >
                <ion-slides>
                    <ion-slide style="display: block;">
                        <h1>{"ステージ 1"}</h1>
                    </ion-slide>
                    <ion-slide style="display: block;">
                        <h1>{"to be continued..."}</h1>
                    </ion-slide>
                </ion-slides>
            </ion-content>
            <ion-footer>
                <ion-toolbar>
                    <div class="ion-padding">
                        <ion-button expand="full" onclick={handle_click}>{"スタート"}</ion-button>
                    </div>
                </ion-toolbar>
            </ion-footer>
        </>
    }
}
