use crate::board::{Board, BoardConfig};
use crate::math::Vec2;

pub trait Player {
    fn name(&self) -> &str;
    fn board(&self) -> &Board;
    fn points(&self) -> u64;

    fn hit(&self, enemy: &Board) -> Vec2;

    fn add_points(&mut self, points: u64);
    fn got_hit(&self, point: Vec2) -> bool;
}

pub struct HumanPlayer {
    name: String,
    board: Board,
    points: u64,
}

impl HumanPlayer {
    pub fn new(name: String, config: &BoardConfig) -> Self {
        let board = Board::new(config);

        Self {
            name,
            board,
            points: 0,
        }
    }
}

impl Player for HumanPlayer {
    fn name(&self) -> &str {
        &self.name
    }

    fn board(&self) -> &Board {
        &self.board
    }

    fn points(&self) -> u64 {
        self.points
    }

    fn hit(&self, enemy: &Board) -> Vec2 {
        todo!()
    }

    fn add_points(&mut self, points: u64) {
        self.points += points;
    }

    fn got_hit(&self, point: Vec2) -> bool {
        todo!()
    }
}
