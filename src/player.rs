use crate::board::{Board, BoardConfig};

struct Player {
    name: String,
    board: Board,
    points: u64,
}

impl Player {
    pub fn new(name: String, config: BoardConfig) -> Self {
        unimplemented!()
    }
}
