use crate::vec2::Vec2;
use std::f64;
use wasm_bindgen::prelude::*;

pub trait Shape {
    fn center(&self) -> Vec2;
    fn support(&self, dir: &Vec2) -> Vec2;
    fn area(&self) -> f64;
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Polygon {
    vertices: Vec<Vec2>,
}

impl Polygon {
    pub fn new(v: Vec<Vec2>) -> Polygon {
        Polygon { vertices: v }
    }
}

impl Shape for Polygon {
    fn support(&self, dir: &Vec2) -> Vec2 {
        if self.vertices.len() < 1 {
            panic!("Not a polygon!")
        }

        let mut max_dist: f64 = f64::MIN;
        let mut max_vertex: Vec2 = Vec2::new(0.0, 0.0);

        for v in &self.vertices {
            let dist: f64 = Vec2::dot(v, dir);
            if dist > max_dist {
                max_dist = dist;
                max_vertex = *v;
            }
        }

        max_vertex
    }

    fn area(&self) -> f64 {
        let mut area = 0.0;
        let mut i = self.vertices.len() - 1;
        for j in 0..self.vertices.len() {
            area += self.vertices[i].x() * self.vertices[j].y()
                - self.vertices[j].x() * self.vertices[i].y();

            i = j;
        }

        0.5 * area
    }

    fn center(&self) -> Vec2 {
        let mut cx = 0.0;
        let mut cy = 0.0;
        let mut i = self.vertices.len() - 1;
        for j in 0..self.vertices.len() {
            let c = self.vertices[i].x() * self.vertices[j].y()
                - self.vertices[j].x() * self.vertices[i].y();
            cx += c * (self.vertices[i].x() + self.vertices[j].x());
            cy += c * (self.vertices[i].y() + self.vertices[j].y());

            i = j;
        }

        cx *= 1.0 / (6.0 * self.area());
        cy *= 1.0 / (6.0 * self.area());
        Vec2::new(cx, cy)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Circle {
    center: Vec2,
    radius: f64,
}

impl Circle {
    pub fn new(center: Vec2, radius: f64) -> Circle {
        Circle { center, radius }
    }
}

impl Shape for Circle {
    fn support(&self, dir: &Vec2) -> Vec2 {
        self.center + self.radius * dir.normalize()
    }

    fn center(&self) -> Vec2 {
        self.center
    }

    fn area(&self) -> f64 {
        self.radius * self.radius * f64::consts::PI
    }
}

/// Shape unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec2::Vec2;

    #[test]
    fn test_polygon_area() {
        let a = Polygon::new(vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(5.0, 0.0),
            Vec2::new(5.0, 5.0),
            Vec2::new(0.0, 5.0),
        ]);

        assert_eq!(a.area(), 25.0);
    }

    #[test]
    fn test_polygon_center() {
        let a = Polygon::new(vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(5.0, 0.0),
            Vec2::new(5.0, 5.0),
            Vec2::new(0.0, 5.0),
        ]);

        assert_eq!(a.center(), Vec2::new(2.5, 2.5));
    }

    #[test]
    fn test_circle_center() {
        let a = Circle::new(Vec2::new(1.0, 1.0), 1.0);

        assert_eq!(a.center(), Vec2::new(1.0, 1.0));
    }

    #[test]
    fn test_circle_area() {
        let a = Circle::new(Vec2::new(1.0, 1.0), 1.0);

        assert_eq!(a.area(), f64::consts::PI * 1.0 * 1.0);
    }
}
