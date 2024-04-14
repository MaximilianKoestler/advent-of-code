//! Advent of code 2015 day 2 part 2
//! Restrictions for today (same as day 1):
//!   - As many iterator adaptors as possible
//!   - No manual loops
//!   - No external non-std dependencies

use std::io::BufRead;

use day_02_1::{extract_side_lengths, DIMENSIONS};

/// Calculates the wrapping area of a box with the given side lengths.
fn ribbon_length(side_lengths: [u64; DIMENSIONS]) -> u64 {
    // after sorting, the smallest side is defined by the first two elements
    let mut side_lengths = side_lengths.clone();
    side_lengths.sort();

    // smallest side perimeter + volume
    2 * (side_lengths[0] + side_lengths[1]) + side_lengths.iter().product::<u64>()
}

/// Calculates the wrapping area of a box with the given dimensions as a string.
fn ribbon_length_from_string(side_lengths: &str) -> Option<u64> {
    Some(ribbon_length(extract_side_lengths(side_lengths)?))
}

fn main() {
    let file = std::fs::File::open("../day_02_1/input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let area: u64 = reader
        .lines()
        .map(Result::unwrap)
        .map(|s| ribbon_length_from_string(&s))
        .map(Option::unwrap)
        .sum();
    println!("Ribbon length: {}", area);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ribbon_length() {
        assert_eq!(ribbon_length([2, 3, 4]), 34);
        assert_eq!(ribbon_length([1, 1, 10]), 14);
        assert_eq!(ribbon_length([10, 1, 1]), 14);
        assert_eq!(ribbon_length([1, 10, 1]), 14);
    }

    #[test]
    fn test_ribbon_length_from_string() {
        assert_eq!(ribbon_length_from_string("2x3x4"), Some(34));
        assert_eq!(ribbon_length_from_string("1x1x10"), Some(14));
        assert!(ribbon_length_from_string("1x1x").is_none());
    }
}
