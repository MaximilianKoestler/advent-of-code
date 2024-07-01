//! Advent of code 2015 day 13 part 2

use day_13_1::{build_name_map, build_neighborhood_matrix, compute_optimal_seating, Information};
use std::io::BufRead;

fn main() {
    let file = std::fs::File::open("../day_13_1/input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let entries: Vec<Information> = reader
        .lines()
        .map(Result::unwrap)
        .map(|s| Information::try_from(&*s).unwrap())
        .collect();

    let mut name_map = build_name_map(&entries);

    // add myself
    name_map.insert("me".into(), name_map.len());

    // Note: We can leave the matrix as is, since the new row and column will be filled with None.
    //       The optimization algorithm will treat None as 0 which is exactly what we want.
    let matrix = build_neighborhood_matrix(&entries, &name_map);

    let (_, happiness) = compute_optimal_seating(&matrix).unwrap();
    println!("Optimal happiness: {happiness}");
}
