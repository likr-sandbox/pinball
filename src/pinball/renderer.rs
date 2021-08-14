use crate::pinball::game::{Ball, Game, Plunger};
use std::f64::consts::PI;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub fn draw(game: &Game, canvas: &HtmlCanvasElement) {
    let cw = canvas.width() as f64;
    let ch = canvas.height() as f64;
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    let gw = game.width;
    let gh = game.height;
    let margin = 30.;

    context.save();
    context.clear_rect(0., 0., cw, ch);
    context.set_fill_style(&"#102".into());
    context.fill_rect(0., 0., cw, ch);

    let s = (cw / (gw + 2. * margin)).min(ch / (gh + 2. * margin));
    context.translate(cw / 2., ch / 2.).ok();
    context.scale(s, -s).ok();
    context.translate(-gw / 2., -gh / 2.).ok();

    draw_board(&game, &context);
    let d = game.plunger.d;
    draw_plunger(&game.plunger, d, &context);
    draw_ball(&game.ball, d, &context);

    context.restore();
}

fn draw_plunger(plunger: &Plunger, d: f64, context: &CanvasRenderingContext2d) {
    context.save();
    context.set_fill_style(&"#C0C0C0".into());
    context.fill_rect(plunger.x, plunger.y - plunger.h - d, plunger.w, plunger.h);
    context.restore();
}

fn draw_ball(ball: &Ball, d: f64, context: &CanvasRenderingContext2d) {
    context.save();
    context.set_fill_style(&"#f00".into());
    context.begin_path();
    context.arc(ball.x, ball.y - d, ball.r, 0., 2. * PI).ok();
    context.close_path();
    context.fill();
    context.restore();
}

fn draw_board(game: &Game, context: &CanvasRenderingContext2d) {
    context.save();
    context.set_fill_style(&"#fff".into());
    context.fill_rect(0., 0., game.width, game.height);
    context.set_fill_style(&"#8b4513".into());
    let s = 20.;
    context.fill_rect(0., 0., s, game.height);
    context.fill_rect(0., 0., 520., s);
    context.fill_rect(0., game.height - s, game.width, s);
    context.fill_rect(game.width - s, 0., s, game.height);
    context.fill_rect(500., 0., s, 400.);
    context.restore();
}
