//! Advent of code 2015 day 18 part 1

use std::io::BufRead;

use day_18_1::LightGrid;

fn main() {
    let file = std::fs::File::open("input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);
    let grid = LightGrid::from_lines(reader.lines().map(Result::unwrap)).unwrap();

    let grid = (0..100).fold(grid, |grid, _| grid.step());
    println!("Lights on: {}", grid.count_on());
}
