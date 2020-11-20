use std::ops;

extern crate overload;
use overload::overload;
use wasm_bindgen::prelude::*;

/// A 2d vector
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Vec2 {
    #[wasm_bindgen(constructor)]
    /// Creates a new Vec2
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x, y }
    }

    /// Creates a new Vec2 with the same value for x and y
    pub fn splat(v: f64) -> Vec2 {
        Vec2 { x: v, y: v }
    }

    /// Length^2
    pub fn len_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    /// Length
    pub fn len(&self) -> f64 {
        self.len_sq().sqrt()
    }

    /// Angle needed to rotate this vector to lay on another vector.
    pub fn angle_to(&self, other: &Vec2) -> f64 {
        ((self.x * other.x + self.y * other.y) / (self.len() * other.len())).acos()
    }

    /// Angle in radians
    pub fn angle(&self) -> f64 {
        self.y.atan2(self.x)
    }

    /// Rotate
    pub fn rotate(&self, angle: f64) -> Vec2 {
        Vec2 {
            x: angle.cos() * self.x - angle.sin() * self.y,
            y: angle.sin() * self.x + angle.cos() * self.y,
        }
    }

    /// Normalize
    pub fn normalize(&self) -> Vec2 {
        self / self.len()
    }

    pub fn lerp(v: &Vec2, w: &Vec2, percent: f64) -> Vec2 {
        (v + w) * percent
    }

    pub fn dot(v: &Vec2, w: &Vec2) -> f64 {
        v.x * w.x + v.y * w.y
    }

    pub fn cross(v: &Vec2, w: &Vec2) -> f64 {
        v.x * w.y - v.y * w.x
    }

    pub fn add(&self, other: &Vec2) -> Vec2 {
        self + other
    }

    pub fn sub(&self, other: &Vec2) -> Vec2 {
        self - other
    }

    pub fn mul(&self, other: f64) -> Vec2 {
        self * other
    }

    pub fn div(&self, other: f64) -> Vec2 {
        self / other
    }
}

overload!((a: ?Vec2) + (b: ?Vec2) -> Vec2 { Vec2 { x: a.x + b.x, y: a.y + b.y } });
overload!((a: ?Vec2) - (b: ?Vec2) -> Vec2 { Vec2 { x: a.x - b.x, y: a.y - b.y } });

overload!((a: ?Vec2) * (b: ?Vec2) -> Vec2 { Vec2 { x: a.x * b.x, y: a.y * b.y } });
overload!((a: ?f64)  * (b: ?Vec2) -> Vec2 { Vec2 { x: a * b.x, y: a * b.y } });
overload!((a: ?Vec2) * (b: ?f64)  -> Vec2 { Vec2 { x: a.x * b, y: a.y * b } });

overload!((a: ?Vec2) / (b: ?Vec2) -> Vec2 { Vec2 { x: a.x / b.x, y: a.y / b.y } });
overload!((a: ?f64)  / (b: ?Vec2) -> Vec2 { Vec2 { x: a / b.x, y: a / b.y } });
overload!((a: ?Vec2) / (b: ?f64)  -> Vec2 { Vec2 { x: a.x / b, y: a.y / b } });

overload!(- (a: ?Vec2) -> Vec2 { Vec2 { x: -a.x, y: -a.y } });

/// Vec2 unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        assert_eq!(
            Vec2 { x: 1.0, y: 3.0 } + Vec2 { x: 2.0, y: -2.0 },
            Vec2 { x: 3.0, y: 1.0 }
        );
        assert_eq!(
            Vec2 { x: 1.0, y: 3.0 } - Vec2 { x: 2.0, y: -2.0 },
            Vec2 { x: -1.0, y: 5.0 }
        );

        assert_eq!(
            Vec2 { x: 1.0, y: 3.0 } * Vec2 { x: 2.0, y: -2.0 },
            Vec2 { x: 2.0, y: -6.0 }
        );
        assert_eq!(Vec2 { x: 1.0, y: 3.0 } * 4.0, Vec2 { x: 4.0, y: 12.0 });
        assert_eq!(4.0 * Vec2 { x: 1.0, y: 3.0 }, Vec2 { x: 4.0, y: 12.0 });

        assert_eq!(
            Vec2 { x: 1.0, y: 3.0 } / Vec2 { x: 2.0, y: -2.0 },
            Vec2 { x: 0.5, y: -1.5 }
        );
        assert_eq!(Vec2 { x: 1.0, y: 3.0 } / 4.0, Vec2 { x: 0.25, y: 0.75 });
        assert_eq!(6.0 / Vec2 { x: 1.0, y: 3.0 }, Vec2 { x: 6.0, y: 2.0 });

        assert_eq!(-Vec2 { x: 1.0, y: 3.0 }, Vec2 { x: -1.0, y: -3.0 });
    }

    #[test]
    fn test_splat() {
        assert_eq!(Vec2::splat(1.0), Vec2::new(1.0, 1.0))
    }

    #[test]
    fn test_x() {
        assert_eq!(Vec2::new(1.0, 2.0).x, 1.0)
    }

    #[test]
    fn test_y() {
        assert_eq!(Vec2::new(1.0, 2.0).y, 2.0)
    }

    #[test]
    fn test_len() {
        assert_eq!(Vec2::new(1.0, 0.0).len(), 1.0)
    }

    #[test]
    fn test_len_sq() {
        assert_eq!(Vec2::new(1.0, 0.0).len_sq(), 1.0)
    }

    #[test]
    fn test_angle() {
        assert_eq!(Vec2::new(1.0, 0.0).angle(), 0.0)
    }

    #[test]
    fn test_normalize() {
        assert_eq!(
            Vec2::new(2.0, 2.0).normalize(),
            Vec2::new(0.7071067811865475, 0.7071067811865475)
        )
    }

    #[test]
    fn test_lerp() {
        assert_eq!(
            Vec2::lerp(&Vec2::new(0.0, 0.0), &Vec2::new(1.0, 1.0), 0.5),
            Vec2::new(0.5, 0.5)
        )
    }

    #[test]
    fn test_dot() {
        assert_eq!(Vec2::dot(&Vec2::new(2.0, 3.0), &Vec2::new(5.0, 6.0)), 28.0)
    }

    #[test]
    fn test_cross() {
        assert_eq!(
            Vec2::cross(&Vec2::new(2.0, 3.0), &Vec2::new(5.0, 6.0)),
            -3.0
        )
    }
}
