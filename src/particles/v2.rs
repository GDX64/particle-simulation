use std::ops::{Add, Div, Mul};

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct V2 {
    pub x: f64,
    pub y: f64,
}

impl V2 {
    pub fn new(x: f64, y: f64) -> V2 {
        V2 { x, y }
    }

    pub fn sub(&self, other: &V2) -> V2 {
        V2::new(self.x - other.x, self.y - other.y)
    }

    pub fn add(&self, other: &V2) -> V2 {
        V2::new(self.x + other.x, self.y + other.y)
    }

    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn scalar_mul(&self, scalar: f64) -> V2 {
        V2::new(self.x * scalar, self.y * scalar)
    }

    pub fn norm_sqr(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn normalized(&self) -> V2 {
        let len = self.len();
        if len <= 0.0001 {
            return V2::new(0., 0.);
        }
        V2::new(self.x / len, self.y / len)
    }

    pub fn distance_to(&self, other: &V2) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl Add for V2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<V2> for f64 {
    type Output = V2;

    fn mul(self, rhs: V2) -> Self::Output {
        V2::new(self * rhs.x, self * rhs.y)
    }
}

impl Mul<f64> for V2 {
    type Output = V2;

    fn mul(self, rhs: f64) -> Self::Output {
        V2::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f64> for V2 {
    type Output = V2;

    fn div(self, rhs: f64) -> Self::Output {
        V2::new(self.x / rhs, self.y / rhs)
    }
}

pub trait TreeValue {
    fn position(&self) -> V2;
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn offset_pos(&mut self);
}

pub trait ParticleLike: Sized {
    fn position(&self) -> V2;
    fn velocity(&self) -> V2;
    fn with_position_and_velocity(&self, position: V2, velocity: V2) -> Self;

    fn rk4_integrate(&self, acceleration: V2, dt: f64) -> Self {
        let k1v: V2 = dt * acceleration;
        let k1p: V2 = dt * self.velocity();
        let k2v: V2 = k1v;
        let k2p: V2 = dt * (self.velocity() + k1v / 2.0);
        let k3v: V2 = k1v;
        let k3p: V2 = dt * (self.velocity() + k2v / 2.0);
        let k4v: V2 = k1v;
        let k4p: V2 = dt * (self.velocity() + k3v);
        let new_velocity: V2 = self.velocity() + (k1v + 2.0 * k2v + 2.0 * k3v + k4v) / 6.0;
        let new_position: V2 = self.position() + (k1p + 2.0 * k2p + 2.0 * k3p + k4p) / 6.0;
        self.with_position_and_velocity(new_position, new_velocity)
    }
}
