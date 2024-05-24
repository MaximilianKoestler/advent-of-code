//! Advent of code 2015 day 7 part 1

use std::io::BufRead;

use day_07_1::simulator::evaluate_network;
use day_07_1::wire::Wire;

fn main() {
    let file = std::fs::File::open("input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let signals = evaluate_network(
        reader
            .lines()
            .map(Result::unwrap)
            .map(|line| Wire::try_from(line.as_ref()).unwrap()),
    );

    println!(
        "Signal on wire a: {}",
        signals.unwrap().get_signal("a").unwrap()
    );
}
