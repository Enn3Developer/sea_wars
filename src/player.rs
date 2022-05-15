use crate::board::{Board, BoardConfig};
use crate::math::Vec2;
use crate::Message;

pub trait Player {
    fn name(&self) -> &str;
    fn board(&self) -> &Board;
    fn points(&self) -> u64;

    fn input(&self, enemy: &Board) -> Vec2;

    fn add_points(&mut self, points: u64);
    fn hit(&self, point: Vec2) -> Message;
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

    fn input(&self, enemy: &Board) -> Vec2 {
        todo!()
    }

    fn add_points(&mut self, points: u64) {
        self.points += points;
    }

    fn hit(&self, point: Vec2) -> Message {
        todo!()
    }
}
