use wasm_bindgen::prelude::*;

use crate::geometry::*;
use crate::math::Vec2;

pub struct World {
    objects: Vec<RigidBody>,
    bvh: Vec<usize>,
}

#[wasm_bindgen]
pub struct RigidBody {
    mass: f64,
    density: f64,
    inertia: f64,
    velocity: Vec2,
    acceleration: Vec2,
    shape: Shapes,
}

#[wasm_bindgen]
impl RigidBody {
    pub fn new(mass: f64) -> RigidBody {
        RigidBody {mass, density: 0.0, inertia: 0.0, velocity: Vec2::new(0.0, 0.0), acceleration: Vec2::new(0.0, 0.0), shape: Shapes::Circle(Circle::new(Vec2::new(0.0, 0.0), 0.0))}
    }
}
