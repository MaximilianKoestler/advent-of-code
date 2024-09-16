fn next_base_2_hamming_colexicographical(previous: u32, n: u32) -> Option<u32> {
    let set_bits = previous.count_ones();
    if set_bits >= n {
        None
    } else if previous == 0 {
        Some(1)
    } else if previous.trailing_zeros() == n - set_bits {
        // we need to introduce one more set bit and start from the lsb side
        let next: u32 = (1 << (set_bits + 1)) - 1;
        Some(next)
    } else {
        // we need to move the msb that can move up one position
        let total_bits = u32::try_from(std::mem::size_of_val(&previous) * 8).ok()?;
        let desired_zeros = total_bits - n;

        // remove all leading zeros
        let leading_zeros_front = previous.leading_zeros();
        let next = previous << leading_zeros_front;
        if leading_zeros_front > desired_zeros {
            // the msb can move up one position

            // remove the current msb
            let next = (next << 1) >> 1;

            // shift back to the original position
            let next = next >> leading_zeros_front;

            // add the new msb
            let remaining_bits = total_bits - leading_zeros_front;
            let next = next | (1 << remaining_bits);
            Some(next)
        } else {
            // there are some leading ones we have to deal with first

            // remove all leading ones
            let leading_ones = next.leading_ones();
            let next = next << leading_ones;

            // remove leading zeros, there must be at least one
            let leading_zeros = next.leading_zeros();
            let next = next << leading_zeros;

            // remove the current msb
            let next = (next << 1) >> 1;

            // shift back to the original position
            let next = next >> leading_zeros;

            // add the new msb
            let remaining_bits = total_bits - leading_zeros;
            let next = next | (1 << remaining_bits);

            // add back old leading ones
            let next = next >> leading_ones;
            let old_msbs = (1 << leading_ones) - 1;
            let next = next | (old_msbs << (total_bits - leading_ones - leading_zeros + 1));

            let next = next >> leading_zeros_front;
            Some(next)
        }
    }
}

fn base_2_hamming_colexicographical_with_start(n: u32, start: u32) -> impl Iterator<Item = u32> {
    let mut current: Option<u32> = Some(start);
    Box::new(std::iter::from_fn(move || {
        current.inspect(|&previous| {
            current = next_base_2_hamming_colexicographical(previous, n);
        })
    }))
}

/// Generate the sequence of integers that are first ordered in the following way:
///   - Integers are ordered by the number of 1 in their binary representation (Hamming weight)
///   - Elements of the same Hamming weight are ordered by reverse colexicographical order.
///
/// This is part of the sequence A359941 in the OEIS.
#[allow(dead_code)]
fn base_2_hamming_colexicographical(n: u32) -> impl Iterator<Item = u32> {
    base_2_hamming_colexicographical_with_start(n, 0)
}

fn masked_values(values: &[u32], mask: u32) -> impl Iterator<Item = u32> + '_ {
    let n = values.len();
    (0..n).filter_map(move |i| {
        if mask & (1 << i) != 0 {
            Some(values[i])
        } else {
            None
        }
    })
}

#[must_use]
pub fn apply_mask(values: &[u32], mask: u32) -> Vec<u32> {
    masked_values(values, mask).collect()
}

#[must_use]
pub fn count_mask(mask: u32) -> u32 {
    mask.count_ones()
}

#[must_use]
pub fn sum_mask(values: &[u32], mask: u32) -> u32 {
    masked_values(values, mask).sum()
}

#[must_use]
pub fn quantum_entanglement_mask(values: &[u32], mask: u32) -> u64 {
    masked_values(values, mask).map(u64::from).product::<u64>()
}

fn candidates_for_sum(values: &[u32], target: u32) -> impl Iterator<Item = u32> + '_ {
    // values must be sorted in descending order so that this provides masks in ascending size
    // NB: this would have been much easier by just using itertools::powerset

    // to get a head start, find the largest aligned mask smaller or equal to the target
    let mut start = 1;
    while sum_mask(values, start) <= target {
        start <<= 1;
        start |= 1;
    }
    start >>= 1;

    // starting from that mask, find all masks that sum to the target
    base_2_hamming_colexicographical_with_start(u32::try_from(values.len()).unwrap(), start)
        .filter(move |mask| sum_mask(values, *mask) == target)
}

pub fn valid_candidates_for_sum(values: &[u32], target: u32) -> impl Iterator<Item = u32> + '_ {
    // a candidate is only valid if the remaining values can again be used to sum to target twice
    candidates_for_sum(values, target).filter(move |mask| {
        let remaining_values: Vec<_> = values
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| if mask & (1 << i) == 0 { Some(v) } else { None })
            .collect();
        let valid = candidates_for_sum(&remaining_values, target)
            .next()
            .is_some();
        valid
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_2_hamming_colexicographical() {
        // see https://oeis.org/A359941 for the expected values

        let masks: Vec<_> = base_2_hamming_colexicographical(0).collect();
        assert_eq!(masks, vec![0]);

        let masks: Vec<_> = base_2_hamming_colexicographical(1).collect();
        assert_eq!(masks, vec![0, 1]);

        let masks: Vec<_> = base_2_hamming_colexicographical(2).collect();
        assert_eq!(masks, vec![0, 1, 2, 3]);

        let masks: Vec<_> = base_2_hamming_colexicographical(3).collect();
        assert_eq!(masks, vec![0, 1, 2, 4, 3, 5, 6, 7]);

        let masks: Vec<_> = base_2_hamming_colexicographical(4).collect();
        assert_eq!(
            masks,
            vec![0, 1, 2, 4, 8, 3, 5, 9, 6, 10, 12, 7, 11, 13, 14, 15]
        );

        let masks: Vec<_> = base_2_hamming_colexicographical(5).collect();
        assert_eq!(masks.len(), 32);

        let masks: Vec<_> = base_2_hamming_colexicographical(6).collect();
        assert_eq!(masks.len(), 64);
    }

    #[test]
    fn test_candidates_for_sum() {
        let values = vec![11, 10, 9, 8, 7, 5, 4, 3, 2, 1];
        let mut masks = candidates_for_sum(&values, 20);

        // the only possible combination with only 2 parts
        assert_eq!(apply_mask(&values, masks.next().unwrap()), vec![11, 9]);

        // order is arbitrary, but the sum is the same and there are only 3 parts in each sum
        assert_eq!(apply_mask(&values, masks.next().unwrap()), vec![11, 8, 1]);
        assert_eq!(apply_mask(&values, masks.next().unwrap()), vec![11, 7, 2]);
        assert_eq!(apply_mask(&values, masks.next().unwrap()), vec![11, 5, 4]);
        assert_eq!(apply_mask(&values, masks.next().unwrap()), vec![10, 9, 1]);
    }

    #[test]
    fn test_valid_candidates_for_sum() {
        let values = vec![11, 10, 9, 8, 7, 5, 4, 3, 2, 1];
        let mut masks = valid_candidates_for_sum(&values, 20);

        // the only possible combination with only 2 parts
        assert_eq!(apply_mask(&values, masks.next().unwrap()), vec![11, 9]);

        // order is arbitrary, but the sum is the same and there are only 3 parts in each sum
        assert_eq!(apply_mask(&values, masks.next().unwrap()), vec![11, 8, 1]);
        assert_eq!(apply_mask(&values, masks.next().unwrap()), vec![11, 7, 2]);
        assert_eq!(apply_mask(&values, masks.next().unwrap()), vec![11, 5, 4]);
        assert_eq!(apply_mask(&values, masks.next().unwrap()), vec![10, 9, 1]);
    }

    #[test]
    fn test_some_candidates_are_invalid() {
        let values = vec![11, 10, 9, 8, 7, 5, 4, 3, 2, 1];

        let total_candidates: Vec<_> = candidates_for_sum(&values, 20).collect();
        let valid_candidates: Vec<_> = valid_candidates_for_sum(&values, 20).collect();

        assert!(valid_candidates.len() < total_candidates.len());

        // find the invalid candidates
        let invalid_candidates: Vec<_> = total_candidates
            .iter()
            .filter(|&mask| !valid_candidates.contains(mask))
            .collect();

        // [9, 5, 3, 2, 1] is invalid because the remaining values cannot be summed to 20
        assert!(invalid_candidates.contains(&&0b1110100100));
    }
}
