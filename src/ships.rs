use crate::math::{Rotation, Vec2};
use std::cmp::max;
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

    pub fn new_aircraft_carrier() -> Self {
        let mut components = HashMap::new();

        components.insert(Vec2::new(2, 0), 1);
        components.insert(Vec2::new(3, 0), 1);
        components.insert(Vec2::new(0, 1), 1);
        components.insert(Vec2::new(1, 1), 1);
        components.insert(Vec2::new(2, 1), 2);
        components.insert(Vec2::new(3, 1), 2);
        components.insert(Vec2::new(4, 1), 1);
        components.insert(Vec2::new(2, 2), 1);
        components.insert(Vec2::new(3, 2), 1);

        Self::new(11, Rotation::Left, components)
    }

    pub fn new_cruiser() -> Self {
        let mut components = HashMap::new();

        components.insert(Vec2::new(0, 0), 1);
        components.insert(Vec2::new(1, 0), 2);
        components.insert(Vec2::new(2, 0), 1);
        components.insert(Vec2::new(0, 1), 1);
        components.insert(Vec2::new(1, 1), 2);
        components.insert(Vec2::new(2, 1), 1);

        Self::new(8, Rotation::Left, components)
    }

    pub fn new_minesweeper() -> Self {
        let mut components = HashMap::new();

        components.insert(Vec2::new(0, 0), 1);
        components.insert(Vec2::new(1, 0), 1);
        components.insert(Vec2::new(2, 0), 2);
        components.insert(Vec2::new(3, 0), 1);

        Self::new(5, Rotation::Left, components)
    }

    pub fn rotate(mut self, rotation: Rotation) -> Self {
        if rotation != self.rotation {
            let mut raw_components = HashMap::new();
            let mut components = HashMap::new();

            if rotation.value() % 2 != self.rotation.value() % 2 {
                for (vec, value) in self.components {
                    raw_components.insert(vec.swap(), value);
                }
            } else {
                raw_components = self.components;
            }

            if rotation.direction() != self.rotation.direction() {
                let max = Self::size_of_components(&raw_components);
                let max = Vec2::new(max.x() - 1, max.y() - 1);

                for (vec, value) in raw_components {
                    components.insert(Vec2::new(max.x() - vec.x(), max.y() - vec.y()), value);
                }
            } else {
                components = raw_components;
            }

            self.components = components;
        }
        self
    }

    pub fn size(&self) -> Vec2 {
        Self::size_of_components(&self.components)
    }

    pub fn remaining_important_points(&self) -> u64 {
        let mut counter = 0;

        self.components.values().for_each(|value| {
            if *value == 2 {
                counter += 1;
            }
        });

        counter
    }

    pub fn components(&self) -> &HashMap<Vec2, u64> {
        &self.components
    }

    pub fn value_to_char(value: u64, return_one: bool, return_zero: bool) -> char {
        match value {
            0 => {
                if return_zero {
                    '-'
                } else {
                    '\0'
                }
            }
            1 => {
                if return_one {
                    '*'
                } else {
                    '\0'
                }
            }
            2 => 'x',
            3 => '+',
            4 => 'o',
            _ => '\0',
        }
    }

    fn size_of_components(components: &HashMap<Vec2, u64>) -> Vec2 {
        let last = components.iter().last().unwrap().0;
        let mut max_x = last.x();
        let mut max_y = last.y();

        components.iter().for_each(|(vec, _value)| {
            max_x = max(max_x, vec.x());
            max_y = max(max_y, vec.y());
        });

        Vec2::new(max_x + 1, max_y + 1)
    }
}

#[cfg(test)]
mod test {
    use crate::math::{Rotation, Vec2};
    use crate::ships::Ship;

    fn ship_to_content(ship: &Ship) -> String {
        let mut content = String::new();
        let size = ship.size();

        for i in 0..size.y() {
            for j in 0..size.x() {
                if let Some(component) = ship.components.get(&Vec2::new(j, i)) {
                    content.push(Ship::value_to_char(*component, true, true));
                } else {
                    content.push('-');
                }
            }

            content.push('\n');
        }
        content.pop();

        content
    }

    #[test]
    fn ship() {
        let ship = Ship::new_minesweeper();

        assert_eq!(String::from("**x*"), ship_to_content(&ship));
    }

    #[test]
    fn extra_ship() {
        let ship = Ship::new_cruiser();

        assert_eq!(String::from("*x*\n*x*"), ship_to_content(&ship));
    }

    #[test]
    fn rotation_down() {
        let ship = Ship::new_minesweeper().rotate(Rotation::Down);

        assert_eq!(String::from("*\n*\nx\n*"), ship_to_content(&ship));
    }

    #[test]
    fn rotation_right() {
        let ship = Ship::new_minesweeper().rotate(Rotation::Right);

        assert_eq!(String::from("*x**"), ship_to_content(&ship));
    }

    #[test]
    fn rotation_up() {
        let ship = Ship::new_minesweeper().rotate(Rotation::Up);

        assert_eq!(String::from("*\nx\n*\n*"), ship_to_content(&ship));
    }

    #[test]
    fn extra_rotation_down() {
        let ship = Ship::new_cruiser().rotate(Rotation::Down);

        assert_eq!(String::from("**\nxx\n**"), ship_to_content(&ship));
    }

    #[test]
    fn extra_rotation_right() {
        let ship = Ship::new_cruiser().rotate(Rotation::Right);

        assert_eq!(String::from("*x*\n*x*"), ship_to_content(&ship));
    }

    #[test]
    fn extra_rotation_up() {
        let ship = Ship::new_cruiser().rotate(Rotation::Up);

        assert_eq!(String::from("**\nxx\n**"), ship_to_content(&ship));
    }
}
