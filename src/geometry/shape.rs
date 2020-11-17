use crate::math::vec2::Vec2;

use std::f64;

pub trait Shape {
    fn center(&self) -> Vec2;
    fn support(&self, dir: &Vec2) -> Vec2;
    fn area(&self) -> f64;
}
