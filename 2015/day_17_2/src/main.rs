//! Advent of code 2015 day 17 part 2

use std::collections::HashMap;
use std::io::BufRead;

fn count_minimal_combinations(containers: &[u32], target: u32) -> HashMap<usize, usize> {
    type BitVector = u32;

    let mut counts = HashMap::new();
    assert!(containers.len() <= std::mem::size_of::<BitVector>() * 8);
    for bitvector in (1 as BitVector)..(1 << containers.len()) {
        let mut sum = 0;
        for (j, size) in containers.iter().enumerate() {
            if bitvector & (1 << j) != 0 {
                sum += size;
            }
        }
        if sum == target {
            *counts.entry(bitvector.count_ones() as usize).or_insert(0) += 1;
        }
    }
    counts
}

fn main() {
    let file = std::fs::File::open("../day_17_1/input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let containers: Vec<u32> = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let target = 150;
    let counts = count_minimal_combinations(&containers, target);
    let (needed_containers, combinations) = counts.iter().min().map(|(k, v)| (*k, *v)).unwrap();

    println!("Combinations with fewest ({needed_containers}) containers: {combinations}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combinations() {
        let mut counts: Vec<_> = count_minimal_combinations(&[20, 15, 10, 5, 5], 25)
            .into_iter()
            .collect();
        counts.sort();
        assert_eq!(counts, vec![(2, 3), (3, 1)]);
    }
}
