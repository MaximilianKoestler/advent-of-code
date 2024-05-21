//! Advent of code 2015 day 6 part 2
//! Restrictions for today:
//!   - As many iterator adaptors as possible
//!   - No manual loops

use day_06_1::{Action, Instruction};
use std::io::BufRead;

const GRID_SIZE: usize = 1000;
struct LightGrid {
    lights: Vec<Vec<usize>>,
}

impl LightGrid {
    fn new() -> Self {
        Self {
            lights: vec![vec![0; GRID_SIZE]; GRID_SIZE],
        }
    }

    fn apply_instruction(&mut self, instruction: &Instruction) {
        (instruction.start.x..=instruction.end.x).for_each(|x| {
            (instruction.start.y..=instruction.end.y).for_each(|y| match instruction.action {
                Action::TurnOn => self.lights[x][y] += 1,
                Action::TurnOff => self.lights[x][y] = self.lights[x][y].saturating_sub(1),
                Action::Toggle => self.lights[x][y] += 2,
            });
        });
    }

    fn sum_brightness(&self) -> usize {
        self.lights
            .iter()
            .map(|row| row.iter().sum::<usize>())
            .sum::<usize>()
    }
}

fn sum_brightness(instructions: impl Iterator<Item = Instruction>) -> usize {
    let mut grid = LightGrid::new();
    instructions.for_each(|instruction| grid.apply_instruction(&instruction));
    grid.sum_brightness()
}

fn main() {
    let file = std::fs::File::open("../day_06_1/input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let lit_lights = sum_brightness(
        reader
            .lines()
            .map(Result::unwrap)
            .map(|line| Instruction::try_from(line.as_ref()).unwrap()),
    );

    println!("Total brightness: {lit_lights}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use day_06_1::Coordinate;

    #[test]
    fn test_sum_brightness() {
        let mut instructions = vec![Instruction {
            action: Action::TurnOn,
            start: Coordinate { x: 0, y: 0 },
            end: Coordinate { x: 999, y: 999 },
        }];
        assert_eq!(
            sum_brightness(instructions.clone().into_iter()),
            1_000 * 1_000
        );

        instructions.extend(vec![Instruction {
            action: Action::Toggle,
            start: Coordinate { x: 0, y: 0 },
            end: Coordinate { x: 999, y: 0 },
        }]);
        assert_eq!(
            sum_brightness(instructions.clone().into_iter()),
            1_000 * 1_000 + 2_000
        );

        instructions.extend(vec![Instruction {
            action: Action::TurnOff,
            start: Coordinate { x: 499, y: 499 },
            end: Coordinate { x: 500, y: 500 },
        }]);
        assert_eq!(
            sum_brightness(instructions.clone().into_iter()),
            1_000 * 1_000 + 2_000 - 4
        );
    }
}
