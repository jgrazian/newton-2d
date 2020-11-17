use crate::geometry::Shape;
use crate::math::Vec2;
use std::f64;
use wasm_bindgen::prelude::*;

/// gjk return types
#[derive(PartialEq, Debug)]
pub enum GJKResult {
    Intersection(Vec<Vec2>),
    NoIntersection,
}

/// GJK collision test
/// Impl https://blog.hamaluik.ca/posts/building-a-collision-engine-part-1-2d-gjk-collision-detection/
pub fn gjk(shape_a: &dyn Shape, shape_b: &dyn Shape) -> GJKResult {
    let mut vertices: Vec<Vec2> = Vec::new();
    let mut direction: Vec2 = Vec2::new(0.0, 0.0);

    loop {
        match vertices.len() {
            0 => direction = shape_b.center() - shape_a.center(),
            1 => direction = -direction,
            2 => {
                let b: Vec2 = vertices[1];
                let c: Vec2 = vertices[0];

                let cb: Vec2 = b - c;
                let c0: Vec2 = -c;

                direction = triple_product(&cb, &c0, &cb);
            }
            3 => {
                let a: Vec2 = vertices[2];
                let b: Vec2 = vertices[1];
                let c: Vec2 = vertices[0];

                let a0: Vec2 = -a;
                let ab: Vec2 = b - a;
                let ac: Vec2 = c - a;

                let ab_perp: Vec2 = triple_product(&ac, &ab, &ab);
                let ac_perp: Vec2 = triple_product(&ab, &ac, &ac);

                if Vec2::dot(&ab_perp, &a0) > 0.0 {
                    // the origin is outside line ab
                    // get rid of c and add a new support in the direction of abPerp
                    vertices.remove(0);
                    direction = ab_perp;
                } else if Vec2::dot(&ac_perp, &a0) > 0.0 {
                    // the origin is outside line ac
                    // get rid of b and add a new support in the direction of acPerp
                    vertices.remove(1);
                    direction = ac_perp;
                } else {
                    // the origin is inside both ab and ac,
                    // so it must be inside the triangle!
                    return GJKResult::Intersection(vertices);
                }
            }
            _ => panic!("Can't have simplex with _ verts!"),
        }

        // Test to see if we can find a new support
        // If we do continue, if not we did not find an intersection
        let new_vertex: Vec2 = shape_a.support(&direction) - shape_b.support(&-direction);
        vertices.push(new_vertex);
        if Vec2::dot(&direction, &new_vertex) >= 0.0 && direction != Vec2::new(0.0, 0.0) {
            continue;
        } else {
            break;
        }
    }
    GJKResult::NoIntersection
}

/// Vector triple product. Used in GJK
fn triple_product(a: &Vec2, b: &Vec2, c: &Vec2) -> Vec2 {
    let first: f64 = Vec2::cross(a, b);
    let prod = Vec2::new(-c.y() * first, c.x() * first);
    if prod == Vec2::new(0.0, 0.0) {
        return Vec2::new(a.y(), -a.x());
    } else {
        return prod;
    }
}

enum Winding {
    CounterClockwise,
    Clockwise,
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

fn find_closest_edge(winding: &Winding, vertices: &Vec<Vec2>) -> Edge {
    let mut closest_distance: f64 = std::f64::MAX;
    let mut closest_normal: Vec2 = Vec2::new(0.0, 0.0);
    let mut closest_index: usize = 0;

    for i in 0..vertices.len() {
        let mut j = i + 1;
        if j >= vertices.len() {
            j = 0;
        }

        let edge = vertices[j] - vertices[i];

        let mut norm = match winding {
            Winding::Clockwise => Vec2::new(-edge.y(), edge.x()),
            Winding::CounterClockwise => Vec2::new(edge.y(), -edge.x()),
        };
        norm = norm.normalize();

        let dist = Vec2::dot(&norm, &vertices[i]);
        if dist < closest_distance {
            closest_distance = dist;
            closest_normal = norm;
            closest_index = j;
        }
    }
    Edge::new(closest_distance, closest_normal, closest_index)
}

// https://blog.hamaluik.ca/posts/building-a-collision-engine-part-2-2d-penetration-vectors/
pub fn intersect(shape_a: &dyn Shape, shape_b: &dyn Shape) -> Option<Vec2> {
    let mut vertices: Vec<Vec2>;

    let gjk_result = gjk(shape_a, shape_b);
    match gjk_result {
        GJKResult::NoIntersection => return None,
        GJKResult::Intersection(v) => vertices = v,
    };

    let e0 = (vertices[1].x() - vertices[0].x()) * (vertices[1].y() + vertices[0].y());
    let e1 = (vertices[2].x() - vertices[1].x()) * (vertices[2].y() + vertices[1].y());
    let e2 = (vertices[0].x() - vertices[2].x()) * (vertices[0].y() + vertices[2].y());

    let winding = match e0 + e1 + e2 >= 0.0 {
        true => Winding::Clockwise,
        false => Winding::CounterClockwise,
    };
    let mut intersection = Vec2::new(0.0, 0.0);
    for _ in 0..32 {
        let edge = find_closest_edge(&winding, &vertices);
        let support = shape_a.support(&edge.normal) - shape_b.support(&-edge.normal);
        let distance = Vec2::dot(&support, &edge.normal);

        intersection = edge.normal * distance;

        if (distance - edge.distance).abs() <= 0.0001 {
            return Some(intersection);
        } else {
            vertices.insert(edge.index, support);
        }
    }
    Some(intersection)
}

/// gjk unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{Circle, Polygon, Shape};
    use crate::math::Vec2;

    #[test]
    fn test_triple_product() {
        let a = Vec2::new(4.0, 3.0);
        let b = Vec2::new(5.0, 7.0);
        let c = Vec2::new(4.0, 6.0);

        assert_eq!(triple_product(&a, &b, &c), Vec2::new(-78.0, 52.0))
    }

    #[test]
    fn test_gjk_poly_poly() {
        let a = Polygon::new(vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(5.0, 0.0),
            Vec2::new(5.0, 5.0),
            Vec2::new(0.0, 5.0),
        ]);
        let b = Polygon::new(vec![
            Vec2::new(2.0, 2.0),
            Vec2::new(7.0, 2.0),
            Vec2::new(7.0, 7.0),
            Vec2::new(2.0, 7.0),
        ]);
        let c = Polygon::new(vec![
            Vec2::new(10.0, 10.0),
            Vec2::new(15.0, 10.0),
            Vec2::new(15.0, 15.0),
            Vec2::new(10.0, 15.0),
        ]);

        assert_eq!(
            gjk(&a, &b),
            GJKResult::Intersection(vec!(
                Vec2::new(3.0, 3.0),
                Vec2::new(-7.0, -7.0),
                Vec2::new(-7.0, 3.0)
            ))
        );
        assert_eq!(gjk(&a, &c), GJKResult::NoIntersection);
    }

    #[test]
    fn test_gjk_poly_circle() {
        let a = Polygon::new(vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(5.0, 0.0),
            Vec2::new(5.0, 5.0),
            Vec2::new(0.0, 5.0),
        ]);
        let b = Polygon::new(vec![
            Vec2::new(10.0, 10.0),
            Vec2::new(15.0, 10.0),
            Vec2::new(15.0, 15.0),
            Vec2::new(10.0, 15.0),
        ]);
        let c = Circle::new(Vec2::new(6.0, 6.0), 1.5);

        assert_eq!(
            gjk(&a, &c),
            GJKResult::Intersection(vec!(
                Vec2::new(0.06066017177982097, 0.06066017177982097),
                Vec2::new(-7.060660171779821, -7.060660171779821),
                Vec2::new(-7.060660171779821, 0.06066017177982097)
            ))
        );
        assert_eq!(gjk(&b, &c), GJKResult::NoIntersection);
    }

    #[test]
    fn test_gjk_circle_circle() {
        let a = Circle::new(Vec2::new(1.0, 1.0), 1.0);
        let b = Circle::new(Vec2::new(2.0, 2.0), 1.5);
        let c = Circle::new(Vec2::new(6.0, 6.0), 1.0);

        assert_eq!(
            gjk(&a, &b),
            GJKResult::Intersection(vec!(
                Vec2::new(0.7677669529663687, 0.7677669529663687),
                Vec2::new(-2.7677669529663684, -2.7677669529663684),
                Vec2::new(-2.7677669529663684, 0.7677669529663687)
            ))
        );
        assert_eq!(gjk(&a, &c), GJKResult::NoIntersection);
    }

    #[test]
    fn test_intersect_poly_poly() {
        let a = Polygon::new(vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(5.0, 0.0),
            Vec2::new(5.0, 5.0),
            Vec2::new(0.0, 5.0),
        ]);
        let b = Polygon::new(vec![
            Vec2::new(3.0, 4.0),
            Vec2::new(8.0, 4.0),
            Vec2::new(8.0, 9.0),
            Vec2::new(3.0, 9.0),
        ]);
        let c = Polygon::new(vec![
            Vec2::new(10.0, 10.0),
            Vec2::new(15.0, 10.0),
            Vec2::new(15.0, 15.0),
            Vec2::new(10.0, 15.0),
        ]);

        assert_eq!(intersect(&a, &b).unwrap(), Vec2::new(0.0, 1.0));
        assert_eq!(intersect(&a, &c), None);
    }

    #[test]
    fn test_intersect_poly_circle() {
        let a = Polygon::new(vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(5.0, 0.0),
            Vec2::new(5.0, 5.0),
            Vec2::new(0.0, 5.0),
        ]);
        let b = Polygon::new(vec![
            Vec2::new(10.0, 10.0),
            Vec2::new(15.0, 10.0),
            Vec2::new(15.0, 15.0),
            Vec2::new(10.0, 15.0),
        ]);
        let c = Circle::new(Vec2::new(6.0, 6.0), 1.5);

        assert_eq!(
            intersect(&a, &c).unwrap(),
            Vec2::new(0.06139496752482719, 0.060041963125428935)
        );
        assert_eq!(intersect(&b, &c), None);
    }

    #[test]
    fn test_intersect_circle_circle() {
        let a = Circle::new(Vec2::new(1.0, 1.0), 1.0);
        let b = Circle::new(Vec2::new(2.0, 2.0), 1.5);
        let c = Circle::new(Vec2::new(6.0, 6.0), 1.0);

        assert_eq!(
            intersect(&a, &b).unwrap(),
            Vec2::new(0.7630602793663586, 0.7724823693006009)
        );
        assert_eq!(gjk(&a, &c), GJKResult::NoIntersection);
    }
}
