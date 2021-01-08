use wasm_bindgen::prelude::*;
use crate::math::Vec2;

pub mod circle;
pub mod polygon;
pub mod shape;

pub use circle::Circle;
pub use polygon::Polygon;
pub use shape::Shape;

pub enum Shapes {
    Circle(Circle),
    Polygon(Polygon)
}

impl Shape for Shapes {
    fn support(&self, dir: &Vec2) -> Vec2 {
        match self {
            Shapes::Circle(c) => c.support(dir),
            Shapes::Polygon(p) => p.support(dir),
        }
    }

    fn area(&self) -> f64 {
        match self {
            Shapes::Circle(c) => c.area(),
            Shapes::Polygon(p) => p.area(),
        }
    }

    fn center(&self) -> Vec2 {
        match self {
            Shapes::Circle(c) => c.center(),
            Shapes::Polygon(p) => p.center(),
        }
    }
}
