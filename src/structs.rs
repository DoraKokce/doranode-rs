use std::ops::{Add, Div, Mul, Sub};

use pyo3::{IntoPyObject, pyclass};

#[derive(Debug, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
    pub origin: Option<Box<Vector2>>,
}

impl Vector2 {
    pub fn new(x: f32, y: f32, origin: Option<Vector2>) -> Self {
        Self {
            x,
            y,
            origin: origin.map(Box::new),
        }
    }

    pub fn add_origin(&self, origin: Vector2) -> Self {
        Self::new(self.x - origin.x, self.y - origin.y, Some(origin))
    }

    pub fn set_origin(&self, origin: Vector2) -> Self {
        Self::new(self.x, self.y, Some(origin))
    }

    pub fn from_origin(&self) -> Self {
        let origin: Box<Vector2> = self
            .origin
            .clone()
            .unwrap_or_else(|| Box::new(Self::zero()));
        Self::new(self.x + origin.x, self.y + origin.y, None)
    }

    pub fn without_origin(&self) -> Self {
        Self::new(self.x, self.y, None)
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, None)
    }

    pub fn magnitude(&self) -> f32 {
        f32::sqrt((self.x * self.x + self.y * self.y))
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        let self_from = self.from_origin();
        let other = other.from_origin();

        self_from.x == other.x && self_from.y == other.y
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Self) -> Self::Output {
        if self.origin == rhs.origin {
            Self::new(self.x + rhs.x, self.y + rhs.y, self.origin.map(|x| *x))
        } else {
            let result = self.from_origin() + rhs.from_origin();
            if self.origin.is_some() {
                result.add_origin(*self.origin.unwrap())
            } else {
                result.clone()
            }
        }
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.origin == rhs.origin {
            Self::new(self.x - rhs.x, self.y - rhs.y, self.origin.map(|x| *x))
        } else {
            let result = self.from_origin() - rhs.from_origin();
            if self.origin.is_some() {
                result.add_origin(*self.origin.unwrap())
            } else {
                result.clone()
            }
        }
    }
}

impl Mul<Vector2> for Vector2 {
    type Output = f32;

    fn mul(self, rhs: Vector2) -> Self::Output {
        let from = self.from_origin();
        let rhs = rhs.from_origin();
        from.x * rhs.x + from.y * rhs.y
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Self::Output {
        let from = self.from_origin();
        Vector2::new(from.x * rhs, from.y * rhs, None)
    }
}

impl Mul<Vector2> for f32 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output {
        let from = rhs.from_origin();
        Vector2::new(from.x * self, from.y * self, None)
    }
}

impl Div<i32> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: i32) -> Self::Output {
        let from = self.from_origin();
        Vector2::new(from.x / rhs as f32, from.y / rhs as f32, None)
    }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: f32) -> Self::Output {
        let from = self.from_origin();
        Vector2::new(from.x / rhs, from.y / rhs, None)
    }
}

impl From<&Vector2> for raylib::prelude::Vector2 {
    fn from(v: &Vector2) -> Self {
        let v = v.from_origin();
        raylib::prelude::Vector2 { x: v.x, y: v.y }
    }
}

impl From<Vector2> for raylib::prelude::Vector2 {
    fn from(v: Vector2) -> Self {
        let v = v.from_origin();
        raylib::prelude::Vector2 { x: v.x, y: v.y }
    }
}

impl From<Vector2> for raylib::ffi::Vector2 {
    fn from(v: Vector2) -> Self {
        let v = v.from_origin();
        raylib::ffi::Vector2 { x: v.x, y: v.y }
    }
}

impl From<raylib::prelude::Vector2> for Vector2 {
    fn from(v: raylib::prelude::Vector2) -> Self {
        Self {
            x: v.x,
            y: v.y,
            origin: None,
        }
    }
}

impl From<(i32, i32)> for Vector2 {
    fn from(value: (i32, i32)) -> Self {
        Vector2::new(value.0 as f32, value.1 as f32, None)
    }
}

impl From<(f32, f32)> for Vector2 {
    fn from(value: (f32, f32)) -> Self {
        Vector2::new(value.0, value.1, None)
    }
}

impl From<[f32; 2]> for Vector2 {
    fn from(value: [f32; 2]) -> Self {
        Vector2::new(value[0], value[1], None)
    }
}
