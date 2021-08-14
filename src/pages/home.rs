use std::f64::consts::PI;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

fn draw(canvas: HtmlCanvasElement) {
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    context.begin_path();

    // Draw the outer circle.
    context.arc(75.0, 75.0, 50.0, 0.0, PI * 2.0).unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context.arc(60.0, 65.0, 5.0, 0.0, PI * 2.0).unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context.arc(90.0, 65.0, 5.0, 0.0, PI * 2.0).unwrap();

    context.stroke();
}

#[function_component(Home)]
pub fn home() -> Html {
    use_effect(|| {
        if let Some(canvas) = yew::utils::document().get_element_by_id("canvas") {
            let canvas = canvas
                .dyn_into::<HtmlCanvasElement>()
                .map_err(|_| ())
                .unwrap();
            draw(canvas);
        }
        || {}
    });
    html! {
        <>
            <ion-header>
                <ion-toolbar>
                    <ion-buttons>
                        <ion-menu-button />
                    </ion-buttons>
                    <ion-title>{"Home"}</ion-title>
                </ion-toolbar>
            </ion-header>
            <ion-content>
            <ion-item>
                <canvas id="canvas" width="800" height="400" style="width: 100%; height: 100%;" />
                </ion-item>
            </ion-content>
        </>
    }
}
