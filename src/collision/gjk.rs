use std::f64;

use crate::geometry::Shape;
use crate::math::Vec2;

use wasm_bindgen::prelude::*;

/// gjk return types
#[derive(PartialEq, Debug)]
pub enum EvolveResult {
    Intersection,
    StillEvolving,
    NoIntersection,
}

enum Winding {
    CCW,
    CW,
}

struct Edge {
    distance: f64,
    normal: Vec2,
    index: usize,
}

impl Edge {
    fn new(distance: f64, normal: Vec2, index: usize) -> Edge {
        Edge {
            distance,
            normal,
            index,
        }
    }
}

pub struct GJK {
    vertices: Vec<Vec2>,
    direction: Vec2,
}

impl GJK {
    pub fn new() -> GJK {
        GJK {
            vertices: Vec::with_capacity(3),
            direction: Vec2::new(0.0, 0.0),
        }
    }

    pub fn test<T: Shape, U: Shape>(&mut self, a: &T, b: &U) -> bool {
        self.vertices = Vec::with_capacity(3);

        let mut result = EvolveResult::StillEvolving;
        while result == EvolveResult::StillEvolving {
            result = self.evolve_simplex(a, b);
        }
        result == EvolveResult::Intersection
    }

    // https://blog.hamaluik.ca/posts/building-a-collision-engine-part-2-2d-penetration-vectors/
    pub fn intersect<T: Shape, U: Shape>(&mut self, a: &T, b: &U) -> Option<Vec2> {
        if !self.test(a, b) {
            return None;
        }

        let e0 = (self.vertices[1].x - self.vertices[0].x) * (self.vertices[1].y + self.vertices[0].y);
        let e1 = (self.vertices[2].x - self.vertices[1].x) * (self.vertices[2].y + self.vertices[1].y);
        let e2 = (self.vertices[0].x - self.vertices[2].x) * (self.vertices[0].y + self.vertices[2].y);

        let winding = match e0 + e1 + e2 >= 0.0 {
            true => Winding::CW,
            false => Winding::CCW,
        };

        let mut intersection = Vec2::new(0.0, 0.0);

        for _ in 0..16 {
            let edge = self.find_closest_edge(&winding);
            let support = a.support(&edge.normal) - b.support(&-edge.normal);
            let distance = Vec2::dot(&support, &edge.normal);

            intersection = edge.normal * distance;

            if (distance - edge.distance).abs() <= 0.0001 {
                return Some(intersection);
            } else {
                self.vertices.insert(edge.index, support);
            }
        }

        Some(intersection)
    }

    fn add_support<T: Shape, U: Shape>(&mut self, a: &T, b: &U) -> bool {
        let new_vertex = a.support(&self.direction) - b.support(&-self.direction);
        self.vertices.push(new_vertex);
        Vec2::dot(&self.direction, &new_vertex) > 0.0
    }

    pub fn evolve_simplex<T: Shape, U: Shape>(&mut self, shape_a: &T, shape_b: &U) -> EvolveResult {
        match self.vertices.len() {
            0 => self.direction = shape_b.center() - shape_a.center(),
            1 => self.direction = -self.direction,
            2 => {
                let b = self.vertices[1];
                let c = self.vertices[0];

                let cb= b - c;
                let c0 = -c;

                self.direction = Vec2::triple_product(&cb, &c0, &cb);
            }
            3 => {
                let a = self.vertices[2];
                let b = self.vertices[1];
                let c = self.vertices[0];

                let a0 = -a;
                let ab = b - a;
                let ac = c - a;

                let ab_perp = Vec2::triple_product(&ac, &ab, &ab);
                let ac_perp = Vec2::triple_product(&ab, &ac, &ac);

                if Vec2::dot(&ab_perp, &a0) > 0.0 {
                    // the origin is outside line ab
                    // get rid of c and add a new support in the direction of abPerp
                    self.vertices.remove(0);
                    self.direction = ab_perp;
                } else if Vec2::dot(&ac_perp, &a0) > 0.0 {
                    // the origin is outside line ac
                    // get rid of b and add a new support in the direction of acPerp
                    self.vertices.remove(1);
                    self.direction = ac_perp;
                } else {
                    // the origin is inside both ab and ac,
                    // so it must be inside the triangle!
                    return EvolveResult::Intersection;
                }
            }
            _ => panic!("Can't have simplex with _ verts!"),
        };

        match self.add_support(shape_a, shape_b) {
            true => EvolveResult::StillEvolving,
            false => EvolveResult::NoIntersection
        }
    }

    fn find_closest_edge(&self, winding: &Winding) -> Edge {
        let mut closest_distance = std::f64::MAX;
        let mut closest_normal = Vec2::new(0.0, 0.0);
        let mut closest_index = 0;
    
        for i in 0..self.vertices.len() {
            let mut j = i + 1;
            if j >= self.vertices.len() {
                j = 0;
            }
    
            let edge = self.vertices[j] - self.vertices[i];
    
            let mut norm = match winding {
                Winding::CW => Vec2::new(-edge.y, edge.x),
                Winding::CCW => Vec2::new(edge.y, -edge.x),
            };
            norm = norm.normalize();
    
            let dist = Vec2::dot(&norm, &self.vertices[i]);
            if dist < closest_distance {
                closest_distance = dist;
                closest_normal = norm;
                closest_index = j;
            }
        }
        Edge::new(closest_distance, closest_normal, closest_index)
    }
}

/// gjk unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{Circle, Polygon};
    use crate::math::Vec2;

    #[test]
    fn test_gjk_poly_poly() {
        let mut gjk = GJK::new();

        let a = Polygon::new(&vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(5.0, 0.0),
            Vec2::new(5.0, 5.0),
            Vec2::new(0.0, 5.0),
        ]);
        let b = Polygon::new(&vec![
            Vec2::new(2.0, 2.0),
            Vec2::new(7.0, 2.0),
            Vec2::new(7.0, 7.0),
            Vec2::new(2.0, 7.0),
        ]);
        let c = Polygon::new(&vec![
            Vec2::new(10.0, 10.0),
            Vec2::new(15.0, 10.0),
            Vec2::new(15.0, 15.0),
            Vec2::new(10.0, 15.0),
        ]);

        assert_eq!(gjk.test(&a, &b), true);
        assert_eq!(gjk.test(&a, &c), false);
    }

    #[test]
    fn test_gjk_poly_circle() {
        let mut gjk = GJK::new();

        let a = Polygon::new(&vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(5.0, 0.0),
            Vec2::new(5.0, 5.0),
            Vec2::new(0.0, 5.0),
        ]);
        let b = Polygon::new(&vec![
            Vec2::new(10.0, 10.0),
            Vec2::new(15.0, 10.0),
            Vec2::new(15.0, 15.0),
            Vec2::new(10.0, 15.0),
        ]);
        let c = Circle::new(Vec2::new(6.0, 6.0), 1.5);

        assert_eq!(gjk.test(&a, &c), true);
        assert_eq!(gjk.test(&b, &c), false);
    }

    #[test]
    fn test_gjk_circle_circle() {
        let mut gjk = GJK::new();

        let a = Circle::new(Vec2::new(1.0, 1.0), 1.0);
        let b = Circle::new(Vec2::new(2.0, 2.0), 1.5);
        let c = Circle::new(Vec2::new(6.0, 6.0), 1.0);

        assert_eq!(gjk.test(&a, &b), true);
        assert_eq!(gjk.test(&a, &c), false);
    }

    #[test]
    fn test_intersect_poly_poly() {
        let mut gjk = GJK::new();

        let a = Polygon::new(&vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(5.0, 0.0),
            Vec2::new(5.0, 5.0),
            Vec2::new(0.0, 5.0),
        ]);
        let b = Polygon::new(&vec![
            Vec2::new(3.0, 4.0),
            Vec2::new(8.0, 4.0),
            Vec2::new(8.0, 9.0),
            Vec2::new(3.0, 9.0),
        ]);
        let c = Polygon::new(&vec![
            Vec2::new(10.0, 10.0),
            Vec2::new(15.0, 10.0),
            Vec2::new(15.0, 15.0),
            Vec2::new(10.0, 15.0),
        ]);

        assert_eq!(gjk.intersect(&a, &b).unwrap(), Vec2::new(0.0, 1.0));
        assert_eq!(gjk.intersect(&a, &c), None);
    }

    #[test]
    fn test_intersect_poly_circle() {
        let mut gjk = GJK::new();

        let a = Polygon::new(&vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(5.0, 0.0),
            Vec2::new(5.0, 5.0),
            Vec2::new(0.0, 5.0),
        ]);
        let b = Polygon::new(&vec![
            Vec2::new(10.0, 10.0),
            Vec2::new(15.0, 10.0),
            Vec2::new(15.0, 15.0),
            Vec2::new(10.0, 15.0),
        ]);
        let c = Circle::new(Vec2::new(6.0, 6.0), 1.5);

        assert_eq!(
            gjk.intersect(&a, &c).unwrap(),
            Vec2::new(0.06139496752482719, 0.060041963125428935)
        );
        assert_eq!(gjk.intersect(&b, &c), None);
    }

    #[test]
    fn test_intersect_circle_circle() {
        let mut gjk = GJK::new();

        let a = Circle::new(Vec2::new(1.0, 1.0), 1.0);
        let b = Circle::new(Vec2::new(3.0, 1.0), 1.5);
        let c = Circle::new(Vec2::new(6.0, 6.0), 1.0);

        assert_eq!(
            gjk.intersect(&a, &b).unwrap(),
            Vec2::new(0.5000282363673584, 0.0030681733371634417)
        );
        assert_eq!(gjk.intersect(&a, &c), None);
    }
}
