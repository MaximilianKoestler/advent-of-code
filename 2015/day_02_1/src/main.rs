//! Advent of code 2015 day 2 part 1
//! Restrictions for today (same as day 1):
//!   - As many iterator adaptors as possible
//!   - No manual loops
//!   - No external non-std dependencies

use std::io::BufRead;

use day_02_1::{extract_side_lengths, DIMENSIONS};

/// Calculates the wrapping area of a box with the given side lengths.
fn wrapping_area(side_lengths: [u64; DIMENSIONS]) -> u64 {
    // after sorting, the smallest side is defined by the first two elements
    let mut side_lengths = side_lengths.clone();
    side_lengths.sort();

    let mut side_areas = side_lengths
        .iter()
        .zip(side_lengths.iter().cycle().skip(1))
        .map(|(a, b)| a * b)
        .peekable();

    // we need the slack once and the other sides twice each
    side_areas.peek().unwrap().clone() + 2 * side_areas.sum::<u64>()
}

/// Calculates the wrapping area of a box with the given dimensions as a string.
fn wrapping_area_from_string(side_lengths: &str) -> Option<u64> {
    extract_side_lengths(side_lengths).map(wrapping_area)
}

fn main() {
    let file = std::fs::File::open("input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let area: u64 = reader
        .lines()
        .map(Result::unwrap)
        .map(|s| wrapping_area_from_string(&s))
        .map(Option::unwrap)
        .sum();
    println!("Wrapping paper area: {}", area);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrapping_area() {
        assert_eq!(wrapping_area([2, 3, 4]), 58);
        assert_eq!(wrapping_area([1, 1, 10]), 43);
        assert_eq!(wrapping_area([10, 1, 1]), 43);
        assert_eq!(wrapping_area([1, 10, 1]), 43);
    }

    #[test]
    fn test_wrapping_area_from_string() {
        assert_eq!(wrapping_area_from_string("2x3x4"), Some(58));
        assert_eq!(wrapping_area_from_string("1x1x10"), Some(43));
        assert!(wrapping_area_from_string("1x1x").is_none());
    }
}
