//! Advent of code 2015 day 6 part 1
//! Restrictions for today:
//!   - As many iterator adaptors as possible
//!   - No manual loops

use day_06_1::{Action, Instruction};
use std::io::BufRead;

const GRID_SIZE: usize = 1000;
struct LightGrid {
    lights: Vec<Vec<bool>>,
}

impl LightGrid {
    fn new() -> Self {
        Self {
            lights: vec![vec![false; GRID_SIZE]; GRID_SIZE],
        }
    }

    fn apply_instruction(&mut self, instruction: &Instruction) {
        for x in instruction.start.x..=instruction.end.x {
            for y in instruction.start.y..=instruction.end.y {
                match instruction.action {
                    Action::TurnOn => self.lights[x][y] = true,
                    Action::TurnOff => self.lights[x][y] = false,
                    Action::Toggle => self.lights[x][y] = !self.lights[x][y],
                }
            }
        }
    }

    fn count_lit_lights(&self) -> usize {
        self.lights
            .iter()
            .map(|row| row.iter().filter(|&&light| light).count())
            .sum()
    }
}

fn count_lit_lights(instructions: impl Iterator<Item = Instruction>) -> usize {
    let mut grid = LightGrid::new();
    instructions.for_each(|instruction| grid.apply_instruction(&instruction));
    grid.count_lit_lights()
}

fn main() {
    let file = std::fs::File::open("input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let lit_lights = count_lit_lights(
        reader
            .lines()
            .map(Result::unwrap)
            .map(|line| Instruction::try_from(line.as_ref()).unwrap()),
    );

    println!("Lit lights: {}", lit_lights);
}

#[cfg(test)]
mod tests {
    use super::*;
    use day_06_1::Coordinate;

    #[test]
    fn test_count_lit_lights() {
        let mut instructions = vec![Instruction {
            action: Action::TurnOn,
            start: Coordinate { x: 0, y: 0 },
            end: Coordinate { x: 999, y: 999 },
        }];
        assert_eq!(
            count_lit_lights(instructions.clone().into_iter()),
            1_000 * 1_000
        );

        instructions.extend(vec![Instruction {
            action: Action::Toggle,
            start: Coordinate { x: 0, y: 0 },
            end: Coordinate { x: 999, y: 0 },
        }]);
        assert_eq!(
            count_lit_lights(instructions.clone().into_iter()),
            (1_000 - 1) * 1_000
        );

        instructions.extend(vec![Instruction {
            action: Action::Toggle,
            start: Coordinate { x: 499, y: 499 },
            end: Coordinate { x: 500, y: 500 },
        }]);
        assert_eq!(
            count_lit_lights(instructions.clone().into_iter()),
            (1_000 - 1) * 1_000 - 4
        );
    }
}
