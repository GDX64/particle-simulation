use nalgebra::Vector2;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

struct Ball {
    mass: f64,
    radius: f64,
    position: Vector2<f64>,
    velocity: Vector2<f64>,
}

struct Link {
    length: f64,
    first: Option<usize>,
    second: usize,
}

#[wasm_bindgen]
pub struct Pendulum {
    balls: [Ball; 3],
    links: [Link; 3],
}

#[wasm_bindgen]
impl Pendulum {
    pub fn new() -> Self {
        let ball1 = Ball {
            mass: 1.0,
            radius: 0.1,
            position: Vector2::new(1.0, 0.0),
            velocity: Vector2::new(0.0, 0.0),
        };
        let ball2 = Ball {
            mass: 1.0,
            radius: 0.1,
            position: Vector2::new(2.0, 0.),
            velocity: Vector2::new(0.0, 0.0),
        };
        let ball3 = Ball {
            mass: 1.0,
            radius: 0.1,
            position: Vector2::new(3.0, 0.0),
            velocity: Vector2::new(0.0, 0.0),
        };
        let link1 = Link {
            length: 1.0,
            first: None,
            second: 0,
        };
        let link2 = Link {
            length: 1.0,
            first: Some(0),
            second: 1,
        };
        let link3 = Link {
            length: 1.0,
            first: Some(1),
            second: 2,
        };
        Self {
            balls: [ball1, ball2, ball3],
            links: [link1, link2, link3],
        }
    }

    pub fn evolve(&mut self) {
        let dt = 0.01;
        //update velocities
        self.balls.iter_mut().for_each(|ball| {
            ball.velocity += Vector2::new(0.0, -9.8) * dt;
        });
        //update positions
        let mut new_positions: Vec<Vector2<f64>> = self
            .balls
            .iter()
            .map(|ball| {
                return ball.position + ball.velocity * dt;
            })
            .collect();

        //fix constraints
        self.links.iter_mut().for_each(|link| {
            match link.first {
                Some(first) => {
                    let delta: Vector2<f64> = new_positions[link.second] - new_positions[first];
                    let length = delta.norm();
                    let error = length - link.length;
                    let both_masses = self.balls[first].mass + self.balls[link.second].mass;
                    let error_ball_1 = error * self.balls[first].mass / both_masses;
                    let error_ball_2 = error * self.balls[link.second].mass / both_masses;

                    let delta_normalized: Vector2<f64> = delta.normalize();
                    let new_position_1 = new_positions[first] + delta_normalized * error_ball_1;
                    let new_position_2 =
                        new_positions[link.second] - delta_normalized * error_ball_2;
                    new_positions[first] = new_position_1;
                    new_positions[link.second] = new_position_2;
                }
                None => {
                    let new_position_2 = new_positions[link.second].normalize() * link.length;
                    new_positions[link.second] = new_position_2;
                }
            }
        });

        //update velocities and positions
        self.balls.iter_mut().enumerate().for_each(|(i, ball)| {
            ball.velocity = (new_positions[i] - ball.position) / dt;
            ball.position = new_positions[i];
        });
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.set_stroke_style(&JsValue::from_str("white"));
        ctx.set_fill_style(&JsValue::from_str("white"));
        // ctx.fill_rect(0.0, 0.0, 100.0, 100.0);
        let scale_factor = 100.0;
        let to_pixel_matrix = nalgebra::Matrix2::identity() * scale_factor;
        self.links.iter().for_each(|link| {
            let position = if let Some(first) = link.first {
                let ball1 = &self.balls[first];
                ball1.position
            } else {
                Vector2::new(0.0, 0.0)
            };
            let position_1 = to_pixel_matrix * position;
            let position_2 = to_pixel_matrix * self.balls[link.second].position;
            ctx.begin_path();
            ctx.move_to(position_1.x, position_1.y);
            ctx.line_to(position_2.x, position_2.y);
            ctx.stroke();
            ctx.begin_path();
            ctx.arc(
                position_2.x,
                position_2.y,
                self.balls[link.second].radius * scale_factor,
                0.0,
                2.0 * std::f64::consts::PI,
            )
            .unwrap();
            ctx.fill();
        });
    }
}
