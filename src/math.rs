use rand::{thread_rng, Rng};

#[derive(Eq, PartialEq)]
pub enum Rotation {
    Left,
    Down,
    Right,
    Up,
}

impl Rotation {
    pub fn random() -> Self {
        let value = thread_rng().gen_range(0..=3);
        match value {
            0 => Self::Left,
            1 => Self::Down,
            2 => Self::Right,
            3 => Self::Up,
            _ => unreachable!(),
        }
    }

    pub fn value(&self) -> u64 {
        match self {
            Self::Left => 0,
            Self::Down => 1,
            Self::Right => 2,
            Self::Up => 3,
        }
    }

    pub fn direction(&self) -> u64 {
        match self {
            Self::Left => 1,
            Self::Down => 1,
            Self::Right => 0,
            Self::Up => 0,
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
pub struct Vec2 {
    x: u64,
    y: u64,
}

impl Vec2 {
    pub fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }

    pub fn swap(&self) -> Self {
        Self {
            x: self.y,
            y: self.x,
        }
    }

    pub fn x(&self) -> u64 {
        self.x
    }

    pub fn y(&self) -> u64 {
        self.y
    }

    pub fn set_x(&mut self, x: u64) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: u64) {
        self.y = y;
    }

    pub fn sum(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
