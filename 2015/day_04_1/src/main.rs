//! Advent of code 2015 day 4 part 1
//! Restrictions for today (same before):
//!   - As many iterator adaptors as possible
//!   - No manual loops

use std::ops::Deref;

const LEADING_ZERO_NIBBLES: u32 = 5;
fn valid_coin(hash: &[u8; 16]) -> bool {
    u128::from_be_bytes(*hash).leading_zeros() >= LEADING_ZERO_NIBBLES * 4
}

fn smallest_suffix(secret_key: &str) -> Option<u64> {
    (0u64..)
        .map(|i| (i, format!("{}{}", secret_key, i)))
        .map(|(i, s)| (i, md5::compute(s)))
        .find(|(_, s)| valid_coin(s.deref()))
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
        assert_eq!(smallest_suffix("ffgimz"), Some(42));
        assert_eq!(smallest_suffix("ccqsvy"), Some(1337));
    }

    #[test]
    #[ignore = "Takes too long"]
    fn test_smallest_suffix() {
        assert_eq!(smallest_suffix("abcdef"), Some(609043));
        assert_eq!(smallest_suffix("pqrstuv"), Some(1048970));
    }
}
