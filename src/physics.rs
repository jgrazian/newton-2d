use crate::geometry::*;
use crate::math::Vec2;

struct World {
    objects: Vec<RigidBodyTypes>,
    bvh: Vec<usize>,
}

struct RigidBody<T: Shape> {
    mass: f64,
    density: f64,
    inertia: f64,
    velocity: Vec2,
    acceleration: Vec2,
    shape: T,
}

enum RigidBodyTypes {
    Circle(RigidBody<Circle>),
    Polygon(RigidBody<Polygon>),
}
