//! Advent of code 2015 day 17 part 1
use std::io::BufRead;

fn count_combinations(containers: &[u32], target: u32) -> usize {
    type BitVector = u32;

    let mut count = 0;
    assert!(containers.len() <= std::mem::size_of::<BitVector>() * 8);
    for bitvector in (1 as BitVector)..(1 << containers.len()) {
        let mut sum = 0;
        for (j, size) in containers.iter().enumerate() {
            if bitvector & (1 << j) != 0 {
                sum += size;
            }
        }
        if sum == target {
            count += 1;
        }
    }
    count
}

fn main() {
    let file = std::fs::File::open("input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let containers: Vec<u32> = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let target = 150;
    let count = count_combinations(&containers, target);

    println!("Combinations: {count}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combinations() {
        assert_eq!(count_combinations(&[20, 15, 10, 5, 5], 25), 4);
    }
}
