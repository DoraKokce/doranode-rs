use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32,
    pub origin: Option<Box<Vector2>>,
}

impl Vector2 {
    pub fn new(x: i32, y: i32, origin: Option<Vector2>) -> Self {
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
        Self::new(0, 0, None)
    }

    pub fn magnitude(&self) -> f32 {
        f32::sqrt((self.x * self.x + self.y * self.y) as f32)
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

impl Mul for Vector2 {
    type Output = i32;

    fn mul(self, rhs: Vector2) -> Self::Output {
        let from = self.from_origin();
        let rhs = rhs.from_origin();
        from.x * rhs.x + from.y * rhs.y
    }
}

impl Div<i32> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: i32) -> Self::Output {
        let from = self.from_origin();
        Vector2::new(from.x / rhs, from.y / rhs, None)
    }
}

impl From<&Vector2> for raylib::prelude::Vector2 {
    fn from(v: &Vector2) -> Self {
        let v = v.from_origin();
        raylib::prelude::Vector2 {
            x: v.x as f32,
            y: v.y as f32,
        }
    }
}

impl From<Vector2> for raylib::prelude::Vector2 {
    fn from(v: Vector2) -> Self {
        let v = v.from_origin();
        raylib::prelude::Vector2 {
            x: v.x as f32,
            y: v.y as f32,
        }
    }
}

impl From<Vector2> for raylib::ffi::Vector2 {
    fn from(v: Vector2) -> Self {
        let v = v.from_origin();
        raylib::ffi::Vector2 {
            x: v.x as f32,
            y: v.y as f32,
        }
    }
}

impl From<raylib::prelude::Vector2> for Vector2 {
    fn from(v: raylib::prelude::Vector2) -> Self {
        Self {
            x: v.x as i32,
            y: v.y as i32,
            origin: None,
        }
    }
}

impl Into<Vector2> for (i32, i32) {
    fn into(self) -> Vector2 {
        Vector2::new(self.0, self.1, None)
    }
}
