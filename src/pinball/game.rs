use std::f64::consts::PI;

pub struct Ball {
    pub x: f64,
    pub y: f64,
    vx: f64,
    vy: f64,
    m: f64,
    pub r: f64,
    fixed: bool,
}

pub struct Pin {
    pub x: f64,
    pub y: f64,
    pub r: f64,
}

pub struct Wall {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

pub struct Hall {
    pub x: f64,
    pub y: f64,
    pub r: f64,
    pub empty: bool,
}

pub struct Plunger {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub d: f64,
    k: f64,
}

pub struct Game {
    pub ball: Ball,
    pub pins: Vec<Pin>,
    pub walls: Vec<Wall>,
    pub halls: Vec<Hall>,
    pub plunger: Plunger,
    pub width: f64,
    pub height: f64,
    g: f64,
    theta: f64,
}

impl Game {
    pub fn new() -> Game {
        let plunger_h = 300.;
        let ball_r = 30.;
        Game {
            ball: Ball {
                x: 550.,
                y: plunger_h + ball_r,
                vx: 0.,
                vy: 0.,
                r: ball_r,
                m: 1.,
                fixed: true,
            },
            pins: vec![],
            walls: vec![],
            halls: vec![],
            plunger: Plunger {
                x: 520.,
                y: plunger_h,
                w: 60.,
                h: plunger_h,
                d: 0.,
                k: 0.01,
            },
            width: 600.,
            height: 800.,
            g: 1.,
            theta: PI / 9.,
        }
    }

    pub fn tick(&mut self) {
        self.update();
    }

    fn update(&mut self) {
        if !self.ball.fixed {
            self.ball.vy -= self.g * self.theta.sin();
            self.ball.x += self.ball.vx;
            self.ball.y += self.ball.vy;
        }
    }

    pub fn pull_plunger(&mut self, dd: f64) {
        self.plunger.d = (self.plunger.d + dd).clamp(0., self.plunger.h);
    }

    pub fn start(&mut self) {
        let k = self.plunger.k;
        let d = self.plunger.d;
        self.plunger.d = 0.;
        self.ball.vy = 0.5 * k * d * d;
        self.ball.fixed = false;
    }
}
