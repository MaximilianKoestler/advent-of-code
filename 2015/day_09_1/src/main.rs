//! Advent of code 2015 day 9 part 1

use day_09_1::{build_distance_matrix, build_index, solve_traveling_salesman, LocationPair};
use std::io::BufRead;

fn main() {
    let file = std::fs::File::open("input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let locations: Vec<LocationPair> = reader
        .lines()
        .map(Result::unwrap)
        .map(|s| LocationPair::try_from(&*s).unwrap())
        .collect();

    let (index_to_city, city_to_index) = build_index(&locations);
    let matrix = build_distance_matrix(&locations, &city_to_index);

    let (distance, path) = solve_traveling_salesman(&matrix).unwrap();
    println!("Shortest distance: {distance}");

    let path: Vec<_> = path.iter().map(|i| index_to_city[*i].clone()).collect();
    println!("Path: {path:?}");
}
