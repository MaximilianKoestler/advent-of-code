pub const DIMENSIONS: usize = 3;

/// Extracts the side lengths from a string.
pub fn extract_side_lengths(side_lengths: &str) -> Option<[u64; DIMENSIONS]> {
    side_lengths
        .split('x')
        .map(str::parse::<u64>)
        .map(Result::ok)
        .collect::<Option<Vec<u64>>>()?
        .try_into()
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_side_lengths() {
        assert_eq!(extract_side_lengths("2x3x4"), Some([2, 3, 4]));
        assert_eq!(extract_side_lengths("1x1x10"), Some([1, 1, 10]));
        assert!(extract_side_lengths("1x1x").is_none());
    }
}
