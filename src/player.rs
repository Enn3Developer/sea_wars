use crate::board::{Board, BoardConfig};
use crate::math::Vec2;

trait Player {
    fn name(&self) -> &str;
    fn board(&self) -> &Board;
    fn points(&self) -> u64;

    fn hit(&self, enemy: &Board) -> Vec2;

    fn add_points(&mut self);
    fn got_hit(&self, point: Vec2) -> bool;
}

struct HumanPlayer {
    name: String,
    board: Board,
    points: u64,
}

impl HumanPlayer {
    pub fn new(name: String, config: BoardConfig) -> Self {
        unimplemented!()
    }
}
