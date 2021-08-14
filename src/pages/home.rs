use crate::pinball::game::Game;
use crate::pinball::renderer::draw;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[function_component(Home)]
pub fn home() -> Html {
    let mousedown_ref = use_ref(|| None);
    let game_ref = use_ref(|| Game::new());
    {
        let game_ref = Rc::clone(&game_ref);
        use_effect(move || {
            let f = Rc::new(RefCell::new(None));
            let g = f.clone();
            *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                if let Some(canvas) = yew::utils::document().get_element_by_id("canvas") {
                    request_animation_frame(f.borrow().as_ref().unwrap());
                    let mut game = game_ref.borrow_mut();
                    let canvas = canvas
                        .dyn_into::<HtmlCanvasElement>()
                        .map_err(|_| ())
                        .unwrap();
                    game.tick();
                    draw(&game, &canvas);
                } else {
                    let _ = f.borrow_mut().take();
                }
            }) as Box<dyn FnMut()>));

            request_animation_frame(g.borrow().as_ref().unwrap());

            || {}
        });
    }

    let handle_mousemove = {
        let mousedown_ref = Rc::clone(&mousedown_ref);
        let game_ref = Rc::clone(&game_ref);
        Callback::from(move |event: MouseEvent| {
            if let Some((x, y)) = *mousedown_ref.borrow() {
                log::info!("{} {}", event.movement_x(), event.movement_y());
                let mut game = game_ref.borrow_mut();
                game.pull_plunger(event.movement_y() as f64)
            }
        })
    };
    let handle_mousedown = {
        let mousedown_ref = Rc::clone(&mousedown_ref);
        Callback::from(move |event| {
            log::info!("mousedown");
            *mousedown_ref.borrow_mut() = Some((0., 0.));
        })
    };
    let handle_mouseup = {
        let mousedown_ref = Rc::clone(&mousedown_ref);
        Callback::from(move |_| {
            log::info!("mouseup");
            *mousedown_ref.borrow_mut() = None;
        })
    };
    let handle_touchmove = {
        let mousedown_ref = Rc::clone(&mousedown_ref);
        let game_ref = Rc::clone(&game_ref);
        Callback::from(move |event: TouchEvent| {
            log::info!("touchmove");
            if let Some((x, y)) = *mousedown_ref.borrow() {
                let mut game = game_ref.borrow_mut();
                game.pull_plunger(1.)
            }
        })
    };
    let handle_touchstart = {
        let mousedown_ref = Rc::clone(&mousedown_ref);
        Callback::from(move |event| {
            log::info!("touchstart");
            *mousedown_ref.borrow_mut() = Some((0., 0.));
        })
    };
    let handle_touchend = {
        let mousedown_ref = Rc::clone(&mousedown_ref);
        let game_ref = Rc::clone(&game_ref);
        Callback::from(move |_| {
            log::info!("touchend");
            *mousedown_ref.borrow_mut() = None;
            let mut game = game_ref.borrow_mut();
            game.start();
        })
    };
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
            <ion-content style="--overflow: hidden;">
                <canvas
                    id="canvas"
                    width="375"
                    height="768"
                    onmousemove={handle_mousemove}
                    onmousedown={handle_mousedown}
                    onmouseup={handle_mouseup}
                    ontouchmove={handle_touchmove}
                    ontouchstart={handle_touchstart}
                    ontouchend={handle_touchend} />
            </ion-content>
        </>
    }
}
