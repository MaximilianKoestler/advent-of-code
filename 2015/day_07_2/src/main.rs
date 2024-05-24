//! Advent of code 2015 day 7 part 1

use std::io::BufRead;

use day_07_1::simulator::evaluate_network;
use day_07_1::wire::Wire;

fn main() {
    let file = std::fs::File::open("../day_07_1/input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let signals = evaluate_network(
        reader
            .lines()
            .map(Result::unwrap)
            .map(|line| Wire::try_from(line.as_ref()).unwrap()),
    );

    let wire_a_part_1 = signals.unwrap().get_signal("a").unwrap();
    let overwrite = format!("{} -> b", wire_a_part_1);

    let file = std::fs::File::open("../day_07_1/input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);
    let signals = evaluate_network(
        reader
            .lines()
            .map(Result::unwrap)
            .map(|line| {
                if line.ends_with("-> b") {
                    overwrite.clone()
                } else {
                    line
                }
            })
            .map(|line| Wire::try_from(line.as_ref()).unwrap()),
    );

    println!(
        "Signal on wire a: {}",
        signals.unwrap().get_signal("a").unwrap()
    );
}
