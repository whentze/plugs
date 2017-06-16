use std::ops::{Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub enum InterpolationMode {
    Linear,
    Flat,
    Quadratic,
}
use self::InterpolationMode::*;

impl Default for InterpolationMode {
    fn default() -> Self {
        InterpolationMode::Linear
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub interpolation_mode: InterpolationMode,
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            ..self
        }
    }
}

impl Mul<f32> for Point {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
            ..self
        }
    }
}

fn interpolate(p1: Point, p2: Point, x: f32) -> f32 {
    match p2.interpolation_mode {
        Linear => p1.y + (x - p1.x) * (p2.y - p1.y) / (p2.x - p1.x),
        Flat => p1.y,
        Quadratic => p2.y + (p1.y - p2.y) * ((p2.x - x) / (p2.x - p1.x)).powi(2),
    }
}

pub struct Envelope {
    pub points: Vec<Point>,
}

impl Envelope {
    pub fn new(points: &[Point]) -> Self {
        use std::cmp::Ordering::*;
        let mut points = points.to_vec();
        points.sort_by(|p1, p2| if p1.x > p2.x {
            Greater
        } else if p1.x < p2.x {
            Less
        } else {
            Equal
        });
        Envelope { points }
    }

    pub fn sample(&self, x: f32) -> f32 {
        let points = &self.points;
        let n = points.len() - 1;
        if x <= points[0].x {
            points[0].y
        } else if x >= points[n].x {
            points[n].y
        } else {
            for i in 0..n {
                if x < points[i + 1].x {
                    return interpolate(points[i], points[i + 1], x);
                }
            }
            panic!("Broken Envelope!");
        }
    }
}
