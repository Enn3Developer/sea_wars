use crate::math::{Rotation, Vec2};
use crate::ships::Ship;
use rand::{thread_rng, Rng};
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
        let mut cells = vec![];
        let mut values = HashMap::new();

        for _ in 0..config.aircrafts {
            let ship = Ship::new_aircraft_carrier().rotate(Rotation::random());
            Board::add_ship(ship, &mut cells, &mut values, &config);
        }

        for _ in 0..config.cruisers {
            let ship = Ship::new_cruiser().rotate(Rotation::random());
            Board::add_ship(ship, &mut cells, &mut values, &config);
        }

        for _ in 0..config.minesweepers {
            let ship = Ship::new_minesweeper().rotate(Rotation::random());
            Board::add_ship(ship, &mut cells, &mut values, &config);
        }

        Self {
            rows: config.rows,
            columns: config.columns,
            cells,
            values,
        }
    }

    fn add_ship(
        ship: Ship,
        cells: &mut Vec<Cell>,
        values: &mut HashMap<Vec2, u64>,
        config: &BoardConfig,
    ) {
        let size = ship.size();

        let mut starting_points = Board::starting_points(&size, values, config);
        let point = starting_points.swap_remove(thread_rng().gen_range(0..starting_points.len()));
        let cell = Cell::new(point, ship);

        for (vec, value) in cell.ship.components() {
            values.insert(vec.sum(&cell.start), *value);
        }

        cells.push(cell);
    }

    fn starting_points(
        size: &Vec2,
        values: &HashMap<Vec2, u64>,
        config: &BoardConfig,
    ) -> Vec<Vec2> {
        let mut points = vec![];

        for i in 0..config.rows {
            for j in 0..config.columns {
                let start = Vec2::new(j, i);
                if Board::can_enter(size, &start, values, config) {
                    points.push(start);
                }
            }
        }

        points
    }

    fn can_enter(
        size: &Vec2,
        start: &Vec2,
        values: &HashMap<Vec2, u64>,
        config: &BoardConfig,
    ) -> bool {
        let end = size.sum(start);
        if end.x() >= config.columns || end.y() >= config.rows || end.x() <= 0 || end.y() <= 0 {
            return false;
        }

        for i in start.y()..(start.y() + size.y()) {
            for j in start.x()..(start.x() + size.x()) {
                if let Some(value) = values.get(&Vec2::new(j, i)) {
                    if *value > 0u64 {
                        return false;
                    }
                }
            }
        }

        true
    }
}
