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

        Self {
            points: 11,
            rotation: Rotation::Left,
            components,
        }
    }

    pub fn new_cruiser() -> Self {
        let mut components = HashMap::new();

        components.insert(Vec2::new(0, 0), 1);
        components.insert(Vec2::new(1, 0), 2);
        components.insert(Vec2::new(2, 0), 1);
        components.insert(Vec2::new(0, 1), 1);
        components.insert(Vec2::new(1, 1), 2);
        components.insert(Vec2::new(2, 1), 1);

        Self {
            points: 8,
            rotation: Rotation::Left,
            components,
        }
    }

    pub fn new_minesweeper() -> Self {
        let mut components = HashMap::new();

        components.insert(Vec2::new(0, 0), 1);
        components.insert(Vec2::new(1, 0), 1);
        components.insert(Vec2::new(2, 0), 2);
        components.insert(Vec2::new(3, 0), 1);

        Self {
            points: 5,
            rotation: Rotation::Left,
            components,
        }
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
        let first = components.keys().collect::<Vec<&Vec2>>()[0];
        let mut max_x = first.x();
        let mut max_y = first.y();

        components.iter().for_each(|(vec, _value)| {
            max_x = max(max_x, vec.x());
            max_y = max(max_y, vec.y());
        });

        Vec2::new(max_x + 1, max_y + 1)
    }
}
