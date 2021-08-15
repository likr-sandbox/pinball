use std::f64::consts::PI;

pub struct Ball {
    pub x: f64,
    pub y: f64,
    vx: f64,
    vy: f64,
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
        let width = 600.;
        let height = 800.;
        let wall_w = 20.;
        let plunger_h = 400.;
        let ball_r = 25.;
        Game {
            ball: Ball {
                x: 550.,
                y: plunger_h + ball_r,
                vx: 0.,
                vy: 0.,
                r: ball_r,
                fixed: true,
            },
            pins: vec![],
            walls: vec![
                Wall {
                    x1: width - 300.,
                    y1: height,
                    x2: width,
                    y2: height - 100.,
                },
                Wall {
                    x1: width - 100.,
                    y1: height,
                    x2: width,
                    y2: height - 300.,
                },
                Wall {
                    x1: 0.,
                    y1: height - 300.,
                    x2: 100.,
                    y2: height,
                },
                Wall {
                    x1: 0.,
                    y1: height - 100.,
                    x2: 300.,
                    y2: height,
                },
                Wall {
                    x1: wall_w,
                    y1: 0.,
                    x2: wall_w,
                    y2: height,
                },
                Wall {
                    x1: width - wall_w,
                    y1: 0.,
                    x2: width - wall_w,
                    y2: height,
                },
                Wall {
                    x1: 0.,
                    y1: wall_w,
                    x2: width,
                    y2: wall_w,
                },
                Wall {
                    x1: 0.,
                    y1: height - wall_w,
                    x2: 600.,
                    y2: height - wall_w,
                },
                Wall {
                    x1: 500.,
                    y1: 0.,
                    x2: 500.,
                    y2: plunger_h,
                },
                Wall {
                    x1: 500. + wall_w,
                    y1: plunger_h,
                    x2: width,
                    y2: plunger_h,
                },
            ],
            halls: vec![],
            plunger: Plunger {
                x: 520.,
                y: plunger_h,
                w: 60.,
                h: plunger_h,
                d: 0.,
                k: 0.0005,
            },
            width,
            height,
            g: 0.1,
            theta: PI / 18.,
        }
    }

    pub fn tick(&mut self) {
        self.update();
    }

    fn update(&mut self) {
        if !self.ball.fixed {
            self.ball.x += self.ball.vx;
            self.ball.y += self.ball.vy;

            for wall in self.walls.iter() {
                let Wall { x1, y1, x2, y2 } = wall;
                let Ball {
                    x, y, vx, vy, r, ..
                } = self.ball;

                let a = y1 - y2;
                let b = x2 - x1;
                let c = x1 * y2 - x2 * y1;
                let d = (a * x + b * y + c).abs() / (a * a + b * b).sqrt();
                if d > r {
                    continue;
                }

                let det = -a * vx - b * vy;
                if det == 0. {
                    continue;
                }

                let e = vx * y - vy * x;
                let cx = (b * e + vx * c) / det;
                let cy = (vy * c - a * e) / det;
                log::info!("{} {} {}", det, cx, cy);
                if cx + 1. < x1.min(*x2)
                    || x1.max(*x2) < cx - 1.
                    || cy + 1. < y1.min(*y2)
                    || y1.max(*y2) < cy - 1.
                {
                    log::info!("{} {} {} {}", x1, x2, y1, y2);
                    continue;
                }

                // let ux = x - cx;
                // let uy = y - cy;
                // self.ball.x += ux / d * r;
                // self.ball.y += uy / d * r;

                let t1 = (y1 - y2).atan2(x1 - x2);
                let t2 = vy.atan2(vx);
                let t3 = 2. * t1 - t2;
                let v = (vx * vx + vy * vy).sqrt();
                self.ball.vx = v * t3.cos();
                self.ball.vy = v * t3.sin();
            }

            self.ball.vy -= self.g * self.theta.sin();
            self.ball.vx *= 0.995;
            self.ball.vy *= 0.995;

            if self.ball.y < 70. {
                self.ball.fixed = true;
            }
        }
    }

    pub fn pull_plunger(&mut self, dd: f64) {
        self.plunger.d = (self.plunger.d + dd).clamp(0., self.plunger.h);
    }

    pub fn start(&mut self) {
        let k = self.plunger.k;
        let d = self.plunger.d;
        self.plunger.d = 0.;
        self.ball.vy = (0.5 * k * d * d).min(10.);
        self.ball.vx = ((self.plunger.d as i64) % 3 - 1) as f64;
        self.ball.fixed = false;
    }

    pub fn moving(&self) -> bool {
        !self.ball.fixed
    }
}
