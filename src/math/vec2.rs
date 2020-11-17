use core::ops::{Add, Div, Mul, Neg, Sub};
use wasm_bindgen::prelude::*;

/// A 2d vector
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Vec2 {
    x: f64,
    y: f64,
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

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
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
        let len = self.len();
        Vec2 {
            x: self.x / len,
            y: self.y / len,
        }
    }

    pub fn lerp(v: &Vec2, w: &Vec2, percent: f64) -> Vec2 {
        Vec2 {
            x: percent * (v.x + w.x),
            y: percent * (v.y + w.y),
        }
    }

    pub fn dot(v: &Vec2, w: &Vec2) -> f64 {
        v.x * w.x + v.y * w.y
    }

    pub fn cross(v: &Vec2, w: &Vec2) -> f64 {
        v.x * w.y - v.y * w.x
    }

    pub fn add(&self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }

    pub fn sub(&self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }

    pub fn mul(&self, other: f64) -> Vec2 {
        Vec2 {
            x: self.x * other,
            y: self.y * other
        }
    }

    pub fn div(&self, other: f64) -> Vec2 {
        Vec2 {
            x: self.x / other,
            y: self.y / other
        }
    }
}

// Addition
impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Add<Vec2> for &Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Add<&Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Add<&Vec2> for &Vec2 {
    type Output = Vec2;

    fn add(self, rhs: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// Subtraction
impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Sub<&Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Sub<Vec2> for &Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Sub<&Vec2> for &Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

/// Componentwise vector multiplication
impl Mul<Vec2> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
impl Mul<&Vec2> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
impl Mul<Vec2> for &Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
impl Mul<&Vec2> for &Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

/// Scalar Multiplication
impl Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f64) -> Vec2 {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl Mul<&f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: &f64) -> Vec2 {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl Mul<f64> for &Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f64) -> Vec2 {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl Mul<&f64> for &Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: &f64) -> Vec2 {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

// Scalar Multiplication
impl Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}
impl Mul<&Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, rhs: &Vec2) -> Vec2 {
        Vec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}
impl Mul<Vec2> for &f64 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}
impl Mul<&Vec2> for &f64 {
    type Output = Vec2;

    fn mul(self, rhs: &Vec2) -> Vec2 {
        Vec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

// Division
impl Div<Vec2> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
impl Div<&Vec2> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
impl Div<Vec2> for &Vec2 {
    type Output = Vec2;

    fn div(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
impl Div<&Vec2> for &Vec2 {
    type Output = Vec2;

    fn div(self, rhs: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

// Scalar Division
impl Div<f64> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f64) -> Vec2 {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl Div<&f64> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: &f64) -> Vec2 {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl Div<f64> for &Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f64) -> Vec2 {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl Div<&f64> for &Vec2 {
    type Output = Vec2;

    fn div(self, rhs: &f64) -> Vec2 {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

// Scalar Division
impl Div<Vec2> for f64 {
    type Output = Vec2;

    fn div(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self / rhs.x,
            y: self / rhs.y,
        }
    }
}
impl Div<&Vec2> for f64 {
    type Output = Vec2;

    fn div(self, rhs: &Vec2) -> Vec2 {
        Vec2 {
            x: self / rhs.x,
            y: self / rhs.y,
        }
    }
}
impl Div<Vec2> for &f64 {
    type Output = Vec2;

    fn div(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self / rhs.x,
            y: self / rhs.y,
        }
    }
}
impl Div<&Vec2> for &f64 {
    type Output = Vec2;

    fn div(self, rhs: &Vec2) -> Vec2 {
        Vec2 {
            x: self / rhs.x,
            y: self / rhs.y,
        }
    }
}

/// Self negation (-)
impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}
impl Neg for &Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

/// Vec2 unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_splat() {
        assert_eq!(Vec2::splat(1.0), Vec2::new(1.0, 1.0))
    }

    #[test]
    fn test_x() {
        assert_eq!(Vec2::new(1.0, 2.0).x(), 1.0)
    }

    #[test]
    fn test_y() {
        assert_eq!(Vec2::new(1.0, 2.0).y(), 2.0)
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
