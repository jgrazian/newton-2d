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

    /// Un-square rooted magnitude
    pub fn len_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    /// Magnitude
    pub fn len(&self) -> f64 {
        self.len_sq().sqrt()
    }

    /// Set magnitude
    pub fn set_len(&self, l: f64) -> Vec2 {
        let len = self.len();
        Vec2 {
            x: self.x * (l / len),
            y: self.y * (l / len),
        }
    }

    /// Angle in radians
    pub fn angle(&self) -> f64 {
        self.y.atan2(self.x)
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
            x: percent * (v.x + w.x()),
            y: percent * (v.y + w.y()),
        }
    }

    pub fn dot(v: &Vec2, w: &Vec2) -> f64 {
        v.x() * w.x() + v.y() * w.y()
    }

    pub fn cross(v: &Vec2, w: &Vec2) -> f64 {
        v.x() * w.y() - v.y() * w.x()
    }
}

/// Componentwise vector addition
impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
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

/// Componentwise scalar addition
impl Add<f64> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: f64) -> Vec2 {
        Vec2 {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

/// Componentwise vector subtraction
impl Sub<&Vec2> for &Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

/// Componentwise scalar subtraction
impl Sub<f64> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: f64) -> Vec2 {
        Vec2 {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

/// Componentwise vector multiplication
impl Mul<&Vec2> for &Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

/// Componentwise scalar multiplication
impl Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f64) -> Vec2 {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

/// Componentwise scalar multiplication
impl Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

/// Componentwise vector division
impl Div<&Vec2> for &Vec2 {
    type Output = Vec2;

    fn div(self, rhs: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

/// Componentwise scalar division
impl Div<f64> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f64) -> Vec2 {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

/// Componentwise scalar division
impl Div<Vec2> for f64 {
    type Output = Vec2;

    fn div(self, rhs: Vec2) -> Vec2 {
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
            Vec2::new(0.70710677, 0.70710677)
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
