use crate::math::{Rotation, Vec2};
use std::collections::HashMap;

pub struct Ship {
    points: u64,
    rotation: Rotation,
    components: HashMap<Vec2, u64>,
}

impl Ship {
    pub fn new(points: u64, rotation: Rotation, components: HashMap<Vec2, u64>) -> Self {
        Self {
            points,
            rotation,
            components,
        }
    }
}
