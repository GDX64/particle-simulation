use nalgebra::Vector2;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

struct DynamicBall {
    mass: f64,
    radius: f64,
    position: Vector2<f64>,
    velocity: Vector2<f64>,
}

enum Ball {
    FIXED { position: Vector2<f64> },
    Dynamic(DynamicBall),
}

impl Ball {
    fn position(&self) -> Vector2<f64> {
        match self {
            Ball::FIXED { position } => *position,
            Ball::Dynamic(ball) => ball.position,
        }
    }

    fn radius(&self) -> f64 {
        match self {
            Ball::FIXED { .. } => 0.0,
            Ball::Dynamic(ball) => ball.radius,
        }
    }

    fn w(&self) -> f64 {
        match self {
            Ball::FIXED { .. } => 0.0,
            Ball::Dynamic(ball) => 1.0 / ball.mass,
        }
    }
}

struct Link {
    length: f64,
}

const SCALE_FACTOR: f64 = 50.0;

#[wasm_bindgen]
pub struct Pendulum {
    balls: Vec<Ball>,
    links: Vec<Link>,
    next_fixed_ball_position: Option<Vector2<f64>>,
}

#[wasm_bindgen]
impl Pendulum {
    pub fn new(balls_num: usize, radius: f64) -> Self {
        let fixed_ball = Ball::FIXED {
            position: Vector2::new(0.0, 0.0),
        };
        let mut balls = vec![fixed_ball];
        (0..balls_num).for_each(|i| {
            let ball = Ball::Dynamic(DynamicBall {
                mass: (i + 1) as f64 / balls_num as f64,
                radius,
                position: Vector2::new(i as f64, 0.0),
                velocity: Vector2::new(0.0, 0.0),
            });
            balls.push(ball);
        });
        let links = (0..balls_num)
            .map(|_| Link { length: 1.0 })
            .collect::<Vec<Link>>();
        Self {
            balls,
            links,
            next_fixed_ball_position: None,
        }
    }

    pub fn update_fixed_ball(&mut self, x: f64, y: f64) {
        self.next_fixed_ball_position = Some(Vector2::new(x / SCALE_FACTOR, y / SCALE_FACTOR));
    }

    pub fn evolve(&mut self, dt: f64, steps: usize) {
        let dt = dt / steps as f64;
        for _ in 0..steps {
            self._evolve(dt);
        }
    }

    fn _evolve(&mut self, dt: f64) {
        //update velocities
        self.balls.iter_mut().for_each(|ball| {
            match ball {
                Ball::FIXED { .. } => {}
                Ball::Dynamic(ball) => {
                    ball.velocity += Vector2::new(0.0, -9.8) * dt;
                }
            }
        });
        //update positions
        let mut new_positions: Vec<Vector2<f64>> = self
            .balls
            .iter()
            .map(|ball| {
                match ball {
                    Ball::FIXED { position } => {
                        if let Some(next_fixed_ball_position) = self.next_fixed_ball_position {
                            next_fixed_ball_position
                        } else {
                            *position
                        }
                    }
                    Ball::Dynamic(ball) => ball.position + ball.velocity * dt,
                }
            })
            .collect();

        self.next_fixed_ball_position = None;

        //fix constraints
        self.links.iter_mut().enumerate().for_each(|(index, link)| {
            let index_ball_1 = index;
            let index_ball_2 = index + 1;
            let ball1 = &self.balls[index_ball_1];
            let ball2 = &self.balls[index_ball_2];
            let delta: Vector2<f64> = new_positions[index_ball_2] - new_positions[index_ball_1];
            let length = delta.norm();
            let error = length - link.length;
            let w1 = ball1.w();
            let w2 = ball2.w();
            let error_ball_1 = error * w1 / (w1 + w2);
            let error_ball_2 = error * w2 / (w1 + w2);

            let delta_normalized = delta.normalize();
            let new_position_1 = new_positions[index_ball_1] + delta_normalized * error_ball_1;
            let new_position_2 = new_positions[index_ball_2] - delta_normalized * error_ball_2;
            new_positions[index_ball_1] = new_position_1;
            new_positions[index_ball_2] = new_position_2;
        });

        //update velocities and positions
        self.balls.iter_mut().enumerate().for_each(|(i, ball)| {
            match ball {
                Ball::FIXED { ref mut position } => {
                    *position = new_positions[i];
                }
                Ball::Dynamic(ball) => {
                    ball.velocity = (new_positions[i] - ball.position) / dt;
                    ball.position = new_positions[i];
                }
            }
        });
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.set_stroke_style(&JsValue::from_str("white"));
        ctx.set_fill_style(&JsValue::from_str("white"));
        // ctx.fill_rect(0.0, 0.0, 100.0, 100.0);
        let to_pixel_matrix = nalgebra::Matrix2::identity() * SCALE_FACTOR;
        self.balls.iter().enumerate().for_each(|(index, ball)| {
            let pos = ball.position();
            let position_1 = to_pixel_matrix * pos;
            if let Some(ball2) = self.balls.get(index + 1) {
                let pos_2 = ball2.position();
                let position_2 = to_pixel_matrix * pos_2;
                ctx.begin_path();
                ctx.move_to(position_1.x, position_1.y);
                ctx.line_to(position_2.x, position_2.y);
                ctx.stroke();
            }
        });
    }
}
