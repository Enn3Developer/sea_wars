use crate::math::Vec2;
use crate::ships::Ship;
use std::collections::HashMap;

pub struct BoardConfig {
    rows: u64,
    columns: u64,
    aircrafts: u64,
    cruisers: u64,
    minesweepers: u64,
}

impl BoardConfig {
    pub fn new(rows: u64, columns: u64, aircrafts: u64, cruisers: u64, minesweepers: u64) -> Self {
        Self {
            rows,
            columns,
            aircrafts,
            cruisers,
            minesweepers,
        }
    }
}

struct Cell {
    start: Vec2,
    ship: Ship,
}

impl Cell {
    pub fn new(start: Vec2, ship: Ship) -> Self {
        Self { start, ship }
    }
}

pub struct Board {
    rows: u64,
    columns: u64,
    cells: Vec<Cell>,
    values: HashMap<Vec2, u64>,
}

impl Board {
    pub fn new(config: BoardConfig) -> Self {
        unimplemented!()
    }
}
