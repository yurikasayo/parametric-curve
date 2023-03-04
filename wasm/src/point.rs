use std::ops::{Add, Sub, Mul};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {x: self.x * other, y: self.y * other}
    }
}

impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point {x: self * other.x, y: self * other.y}
    }
}

impl Point {
    pub fn set(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    pub fn dist_coord(&self, x: f64, y: f64) -> f64 {
        let sq = (x - self.x) * (x - self.x) + (y - self.y) * (y - self.y);
        sq.sqrt()
    }

    pub fn dist(&self, p: Point) -> f64 {
        let sq = (p.x - self.x) * (p.x - self.x) + (p.y - self.y) * (p.y - self.y);
        sq.sqrt()
    }
}