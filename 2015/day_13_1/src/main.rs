//! Advent of code 2015 day 13 part 1

use day_13_1::{build_name_map, build_neighborhood_matrix, compute_optimal_seating, Information};
use std::io::BufRead;

fn main() {
    let file = std::fs::File::open("input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let entries: Vec<Information> = reader
        .lines()
        .map(Result::unwrap)
        .map(|s| Information::try_from(&*s).unwrap())
        .collect();

    let name_map = build_name_map(&entries);
    let matrix = build_neighborhood_matrix(&entries, &name_map);
    let (_, happiness) = compute_optimal_seating(&matrix).unwrap();
    println!("Optimal happiness: {happiness}");
}
