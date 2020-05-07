use crate::vec2::Vec2;
use core::ops::{Add, Neg, Sub};
use wasm_bindgen::prelude::*;

/// A 2d point
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Point2 {
    x: f32,
    y: f32,
}

#[wasm_bindgen]
impl Point2 {
    pub fn new(x: f32, y: f32) -> Point2 {
        Point2 { x, y }
    }

    pub fn splat(v: f32) -> Point2 {
        Point2 { x: v, y: v }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn lerp(p: Point2, q: Point2, percent: f32) -> Point2 {
        Point2 {
            x: percent * (p.x + q.x()),
            y: percent * (p.y + q.y()),
        }
    }

    pub fn to_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

impl Add<Vec2> for Point2 {
    type Output = Point2;

    fn add(self, rhs: Vec2) -> Point2 {
        Point2 {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
        }
    }
}

impl Sub<Point2> for Point2 {
    type Output = Vec2;

    fn sub(self, rhs: Point2) -> Vec2 {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Sub<&Point2> for &Point2 {
    type Output = Vec2;

    fn sub(self, rhs: &Point2) -> Vec2 {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Sub<Vec2> for Point2 {
    type Output = Point2;

    fn sub(self, rhs: Vec2) -> Point2 {
        Point2 {
            x: self.x - rhs.x(),
            y: self.y - rhs.y(),
        }
    }
}

impl Neg for Point2 {
    type Output = Point2;

    fn neg(self) -> Point2 {
        Point2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_splat() {
        assert_eq!(Point2::splat(1.0), Point2::new(1.0, 1.0))
    }

    #[test]
    fn test_x() {
        assert_eq!(Point2::new(1.0, 2.0).x(), 1.0)
    }

    #[test]
    fn test_y() {
        assert_eq!(Point2::new(1.0, 2.0).y(), 2.0)
    }

    #[test]
    fn test_lerp() {
        assert_eq!(
            Point2::lerp(Point2::new(0.0, 0.0), Point2::new(1.0, 1.0), 0.5),
            Point2::new(0.5, 0.5)
        )
    }
}
