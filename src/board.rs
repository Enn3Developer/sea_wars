use crate::math::{Rotation, Vec2};
use crate::ships::Ship;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::fmt::Write as _;

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
    pub fn new(config: &BoardConfig) -> Self {
        let mut cells = vec![];
        let mut values = HashMap::new();

        for _ in 0..config.aircrafts {
            let ship = Ship::new_aircraft_carrier().rotate(Rotation::random());
            Board::add_ship(ship, &mut cells, &mut values, config);
        }

        for _ in 0..config.cruisers {
            let ship = Ship::new_cruiser().rotate(Rotation::Down);
            Board::add_ship(ship, &mut cells, &mut values, config);
        }

        for _ in 0..config.minesweepers {
            let ship = Ship::new_minesweeper().rotate(Rotation::random());
            Board::add_ship(ship, &mut cells, &mut values, config);
        }

        Self {
            rows: config.rows,
            columns: config.columns,
            cells,
            values,
        }
    }

    pub fn string_value(&self, return_one: bool, return_zero: bool) -> String {
        fn int_to_str(mut value: u64) -> String {
            let mut string = String::new();
            let digits = if value > 0 { value.log(26) + 1 } else { 1 };

            for i in (0..digits).rev() {
                let current_base = 26u64.pow(i + 1);
                let digit = value % current_base;
                value /= current_base;

                string.push(char::from_u32((digit + 97) as u32).unwrap());
            }

            string
        }

        let mut content = String::from("   ");

        for i in 0..self.columns {
            let _ = write!(content, "{} ", i + 1);
        }
        content += "\n";

        for i in 0..self.rows {
            let _ = write!(content, "{}: ", int_to_str(i));

            for j in 0..self.columns {
                let global_point = Vec2::new(i, j);
                let mut value = 0;

                if let Some(cell) = self.get_cell(&global_point) {
                    let inner_point = global_point.sub(&cell.start);
                    if let Some(v) = self.values.get(&inner_point) {
                        value = *v;
                    }
                }

                let _ = write!(
                    content,
                    "{} ",
                    Ship::value_to_char(value, return_one, return_zero)
                );
            }

            content += "\n";
        }
        content.pop();

        content
    }

    fn get_cell(&self, point: &Vec2) -> Option<&Cell> {
        for cell in &self.cells {
            if point.is_inside(&cell.start, &cell.ship.size()) {
                return Some(cell);
            }
        }

        None
    }

    fn add_ship(
        mut ship: Ship,
        cells: &mut Vec<Cell>,
        values: &mut HashMap<Vec2, u64>,
        config: &BoardConfig,
    ) {
        let mut starting_points = Board::starting_points(&ship.size(), values, config);

        if starting_points.len() == 0 {
            println!("here");
            let tried = [false, false, false, false];

            while (starting_points.len() == 0) ^ !tried.contains(&false) {
                if !tried[0] {
                    ship = ship.rotate(Rotation::Left);
                    starting_points = Board::starting_points(&ship.size(), values, config);
                    if starting_points.len() > 0 {
                        break;
                    }
                }
                if !tried[1] {
                    ship = ship.rotate(Rotation::Down);
                    starting_points = Board::starting_points(&ship.size(), values, config);
                    if starting_points.len() > 0 {
                        break;
                    }
                }
                if !tried[2] {
                    ship = ship.rotate(Rotation::Right);
                    starting_points = Board::starting_points(&ship.size(), values, config);
                    if starting_points.len() > 0 {
                        break;
                    }
                }
                if !tried[3] {
                    ship = ship.rotate(Rotation::Up);
                    starting_points = Board::starting_points(&ship.size(), values, config);
                    if starting_points.len() > 0 {
                        break;
                    }
                }
            }

            if starting_points.len() == 0 {
                return;
            }
        }

        let point = starting_points.swap_remove(thread_rng().gen_range(0..starting_points.len()));
        let cell = Cell::new(point, ship);

        for (vec, value) in cell.ship.components() {
            values.insert(vec.add(&cell.start), *value);
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
                let start = Vec2::new(i, j);
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
        let end = size.add(start);
        println!("Start: {:?}", start);
        println!("Size: {:?}", size);
        println!("End: {:?}", end);
        println!("Columns: {}; Rows: {}", config.columns, config.rows);
        println!("=============");
        if end.x() > config.columns || end.y() > config.rows {
            return false;
        }

        for i in start.y()..(start.y() + size.y()) {
            for j in start.x()..(start.x() + size.x()) {
                if let Some(value) = values.get(&Vec2::new(j, i)) {
                    if *value != 0 {
                        return false;
                    }
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use crate::board::{Board, BoardConfig};

    #[test]
    fn empty_board() {
        let config = BoardConfig::new(2, 2, 0, 0, 0);
        let board = Board::new(&config);
        let expected = String::from("   1 2 \na: - - \nb: - - ");

        assert_eq!(expected, board.string_value(true, true));
    }

    #[test]
    fn one_boat() {
        let config = BoardConfig::new(3, 2, 0, 1, 0);
        let board = Board::new(&config);
        let expected = String::from("   1 2 \na: * * \nb: x x \nc: * * ");

        assert_eq!(expected, board.string_value(true, true));
    }
}
