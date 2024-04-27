//! Advent of code 2015 day 4 part 2
//! Restrictions for today (same before):
//!   - As many iterator adaptors as possible
//!   - No manual loops

use rayon::prelude::*;
use std::ops::Deref;

const LEADING_ZERO_NIBBLES: u32 = 6;
fn valid_coin(hash: &[u8; 16]) -> bool {
    u128::from_be_bytes(*hash).leading_zeros() >= LEADING_ZERO_NIBBLES * 4
}

// https://github.com/rayon-rs/rayon/issues/359
// https://github.com/rayon-rs/rayon/issues/520
fn smallest_suffix(secret_key: &str) -> Option<u32> {
    (u32::MIN..u32::MAX)
        .into_par_iter()
        .with_min_len(100_000_000)
        .map(|i| (i, format!("{}{}", secret_key, i)))
        .map(|(i, s)| (i, md5::compute(s)))
        .find_first(|(_, s)| valid_coin(s.deref()))
        .map(|(i, _)| i)
}

fn main() {
    let suffix = smallest_suffix("yzbqklnj").unwrap();
    println!("Answer: {}", suffix);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_smallest_suffix_fast() {
        assert_eq!(smallest_suffix("bdmnoopw"), Some(42));
        assert_eq!(smallest_suffix("ccqsvy"), Some(1337));
    }
}
