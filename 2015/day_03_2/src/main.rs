//! Advent of code 2015 day 3 part 2
//! Restrictions for today (same before):
//!   - As many iterator adaptors as possible
//!   - No manual loops
//!   - No external non-std dependencies

use std::io::Read;

use day_03_1::{Direction, Position};

trait WorldMap {
    fn take_step(&mut self, current_position: &mut Position, direction: Direction);
    fn visited_positions(&self) -> usize;
}

struct SetBasedWorldMap {
    visited: std::collections::HashSet<Position>,
}

impl SetBasedWorldMap {
    fn new() -> Self {
        let current_position = Position { x: 0, y: 0 };
        Self {
            visited: vec![current_position].into_iter().collect(),
        }
    }
}

impl WorldMap for SetBasedWorldMap {
    fn take_step(&mut self, current_position: &mut Position, direction: Direction) {
        match direction {
            Direction::North => current_position.y += 1,
            Direction::East => current_position.x += 1,
            Direction::South => current_position.y -= 1,
            Direction::West => current_position.x -= 1,
        }
        self.visited.insert(current_position.clone());
    }

    fn visited_positions(&self) -> usize {
        self.visited.len()
    }
}

fn positions_on_path(map: &mut impl WorldMap, path: impl Iterator<Item = Direction>) -> usize {
    path.scan(
        (0, [Position { x: 0, y: 0 }, Position { x: 0, y: 0 }]),
        |(mover_index, positions), direction| {
            map.take_step(&mut positions[*mover_index], direction);
            *mover_index = (*mover_index + 1) % 2;
            Some(())
        },
    )
    .last();
    map.visited_positions()
}

fn positions_on_path_chars(map: &mut impl WorldMap, path: impl Iterator<Item = char>) -> usize {
    positions_on_path(map, path.map(Direction::try_from).map(Result::unwrap))
}

fn main() {
    let file = std::fs::File::open("../day_03_1/input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let positions = positions_on_path_chars(
        &mut SetBasedWorldMap::new(),
        reader
            .bytes()
            .map(|b| b.and_then(common::ascii_byte_to_char))
            .map(Result::unwrap),
    );

    println!("Positions: {}", positions);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positions_on_path_string() {
        assert_eq!(
            positions_on_path_chars(&mut SetBasedWorldMap::new(), "^v".chars()),
            3
        );
        assert_eq!(
            positions_on_path_chars(&mut SetBasedWorldMap::new(), "^>v<".chars()),
            3
        );
        assert_eq!(
            positions_on_path_chars(&mut SetBasedWorldMap::new(), "^v^v^v^v^v".chars()),
            11
        );
    }
}
