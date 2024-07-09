//! Advent of code 2015 day 14 part 1

use day_14_1::{Reindeer, SimulateUntil};
use std::io::BufRead;

fn main() {
    let file = std::fs::File::open("input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let time = 2503;
    let distance = reader
        .lines()
        .map(Result::unwrap)
        .map(|s| Reindeer::try_from(&*s).unwrap())
        .map(|reindeer| reindeer.simulate(time))
        .max()
        .unwrap();

    println!("Winning distance: {distance}");
}
