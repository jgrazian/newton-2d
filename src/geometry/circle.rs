use wasm_bindgen::prelude::*;

use super::shape::Shape;
use crate::math::vec2::Vec2;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Circle {
    center: Vec2,
    radius: f64,
}

#[wasm_bindgen]
impl Circle {
    #[wasm_bindgen(constructor)]
    pub fn new(center: Vec2, radius: f64) -> Circle {
        Circle { center, radius }
    }

    pub fn center(&self) -> Vec2 {
        self.center
    }

    pub fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

impl Shape for Circle {
    fn support(&self, dir: &Vec2) -> Vec2 {
        self.center + self.radius * dir.normalize()
    }

    fn center(&self) -> Vec2 {
        self.center()
    }

    fn area(&self) -> f64 {
        self.area()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::vec2::Vec2;

    #[test]
    fn test_circle_support() {
        let a = Circle::new(Vec2::new(1.0, 1.0), 1.0);

        assert_eq!(a.support(&Vec2::new(1.0, 0.0)), Vec2::new(2.0, 1.0));
    }

    #[test]
    fn test_circle_center() {
        let a = Circle::new(Vec2::new(1.0, 1.0), 1.0);

        assert_eq!(a.center(), Vec2::new(1.0, 1.0));
    }

    #[test]
    fn test_circle_area() {
        let a = Circle::new(Vec2::new(1.0, 1.0), 1.0);

        assert_eq!(a.area(), std::f64::consts::PI * 1.0 * 1.0);
    }
}
