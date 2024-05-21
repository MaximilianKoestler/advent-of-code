//! Advent of code 2015 day 4 part 2
//! Restrictions for today (same before):
//!   - As many iterator adaptors as possible
//!   - No manual loops

use rayon::prelude::*;

const LEADING_ZERO_NIBBLES: u32 = 6;
fn valid_coin(hash: &[u8; 16]) -> bool {
    u128::from_be_bytes(*hash).leading_zeros() >= LEADING_ZERO_NIBBLES * 4
}

fn smallest_suffix(secret_key: &str) -> Option<u64> {
    const CHUNK_SIZE: u64 = 1024;
    (u32::MIN..u32::MAX)
        .into_par_iter()
        .map(|chunk_id| {
            (0u64..CHUNK_SIZE)
                .map(|i| CHUNK_SIZE * u64::from(chunk_id) + i)
                .map(|i| (i, format!("{secret_key}{i}")))
                .map(|(i, s)| (i, md5::compute(s)))
                .find(|(_, s)| valid_coin(s))
        })
        .find_first(std::option::Option::is_some)
        .flatten()
        .map(|(i, _)| i)

    // Actually I would like this to work:
    // https://github.com/rayon-rs/rayon/issues/359
    // https://github.com/rayon-rs/rayon/issues/520
    // (u32::MIN..u32::MAX)
    //     .into_par_iter()
    //     .with_min_len(100_000)
    //     .map(|i| (i, format!("{secret_key}{}")))i
    //     .map(|(i, s)| (i, md5::compute(s)))
    //     .find_first(|(_, s)| valid_coin(&**s))
    //     .map(|(i, _)| i as u64)
}

fn main() {
    let suffix = smallest_suffix("yzbqklnj").unwrap();
    println!("Answer: {suffix}");
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
