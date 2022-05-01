pub enum Rotation {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}

pub struct Vec2 {
    x: u64,
    y: u64,
}

impl Vec2 {
    pub fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }
}
