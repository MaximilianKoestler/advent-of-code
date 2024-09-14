//! Advent of code 2015 day 24 part 1

use std::io::BufRead;

fn select_for_sum(values: &[u32], target: u32) -> impl Iterator<Item = u32> + '_ {
    // values must be sorted in descending order so that this provides masks in ascending size

    let n = values.len();
    (0..(1 << n)).filter_map(move |mask| {
        let mut sum = 0;
        for i in 0..n {
            if mask & (1 << i) != 0 {
                sum += values[i];
            }
        }
        if sum == target {
            return Some(mask);
        } else {
            return None;
        }
    })
}

fn apply_mask(values: &[u32], mask: u32) -> Vec<u32> {
    let n = values.len();
    let mut result = Vec::new();
    for i in 0..n {
        if mask & (1 << i) != 0 {
            result.push(values[i]);
        }
    }
    result
}

fn main() {
    let file = std::fs::File::open("input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);
    let mut numbers: Vec<_> = reader
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect();

    // ensure that 32 bits are enough to represent masks
    assert!(numbers.len() < 32);

    // sort in ascending order so that we can select the largest numbers first
    numbers.sort_unstable_by(|a, b| b.cmp(a));

    dbg!(numbers);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_for_sum() {
        let values = vec![11, 10, 9, 8, 7, 5, 4, 3, 2, 1];
        let mut masks = select_for_sum(&values, 20);

        // the only possible combination with only 2 parts
        assert_eq!(apply_mask(&values, masks.next().unwrap()), vec![11, 9]);

        // order is arbitrary, but the sum is the same and there are only 3 parts in each sum
        assert_eq!(apply_mask(&values, masks.next().unwrap()), vec![8, 7, 5]);
        assert_eq!(apply_mask(&values, masks.next().unwrap()), vec![9, 7, 4]);
        assert_eq!(apply_mask(&values, masks.next().unwrap()), vec![11, 5, 4]);
    }
}
