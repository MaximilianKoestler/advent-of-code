//! Advent of code 2015 day 14 part 1

use day_14_1::{Reindeer, SimulateUntil};
use itertools::Itertools;
use std::io::BufRead;

fn simulate_many(time: u32, reindeers: &[Reindeer]) -> Vec<u32> {
    let mut points = vec![0; reindeers.len()];
    (1..=time)
        .flat_map(|t| {
            reindeers
                .iter()
                .map(|reindeer| reindeer.simulate(t))
                .enumerate()
                .max_set_by_key(|(_, distance)| *distance)
        })
        .for_each(|(i, _)| {
            points[i] += 1;
        });
    points
}

fn main() {
    let file = std::fs::File::open("../day_14_1/input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let time = 2503;
    let reindeers: Vec<_> = reader
        .lines()
        .map(Result::unwrap)
        .map(|s| Reindeer::try_from(&*s).unwrap())
        .collect();

    let points = simulate_many(time, &reindeers).into_iter().max().unwrap();
    println!("Winning points: {points}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate_many() {
        let reindeers = vec![
            Reindeer {
                name: "Comet".to_string(),
                speed: 14,
                fly_time: 10,
                rest_time: 127,
            },
            Reindeer {
                name: "Dancer".to_string(),
                speed: 16,
                fly_time: 11,
                rest_time: 162,
            },
        ];

        assert_eq!(simulate_many(1000, &reindeers), vec![312, 689]);
    }
}
