//! Advent of code 2015 day 24 part 1

use std::io::BufRead;

use day_24_1::{apply_mask, quantum_entanglement_mask, valid_candidates_for_sum};

fn best_configuration(values: &[u32]) -> Option<u32> {
    let target = values.iter().sum::<u32>() / 3;

    // We can take the first valid candidate because the quantum entanglement will be minimal by
    // construction.
    // If this was not the case, we would need to use the search algorithm below.
    valid_candidates_for_sum(values, target).next()

    // let mut best_mask = None;
    // for mask in valid_candidates_for_sum(values, target) {
    //     if let Some(best_mask_value) = best_mask {
    //         if count_mask(mask) > count_mask(best_mask_value) {
    //             return best_mask;
    //         } else if quantum_entanglement_mask(values, mask)
    //             < quantum_entanglement_mask(values, best_mask_value)
    //         {
    //             best_mask = Some(mask);
    //         }
    //     } else {
    //         best_mask = Some(mask);
    //     }
    // }
    // None
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

    let best = best_configuration(&numbers).unwrap();
    let quantum_entanglement = quantum_entanglement_mask(&numbers, best);

    println!("Best configuration: {:?}", apply_mask(&numbers, best));
    println!("Quantum entanglement: {quantum_entanglement}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use day_24_1::sum_mask;

    #[test]
    fn test_best_configuration() {
        let values = vec![11, 10, 9, 8, 7, 5, 4, 3, 2, 1];
        let best = best_configuration(&values).unwrap();
        assert_eq!(apply_mask(&values, best), vec![11, 9]);
        assert_eq!(sum_mask(&values, best), 20);
        assert_eq!(quantum_entanglement_mask(&values, best), 99);
    }
}
