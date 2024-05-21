//! Advent of code 2015 day 1 part 2
//! Restrictions for today:
//!   - As many iterator adaptors as possible
//!   - No manual loops
//!   - No external non-std dependencies

use std::io::Read;

/// Gets the final floor after following all instructions.
fn get_floor(chars: impl Iterator<Item = char>) -> i32 {
    chars
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum()
}

fn main() {
    let file = std::fs::File::open("input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let floor = get_floor(
        reader
            .bytes()
            .map(|b| b.and_then(common::ascii_byte_to_char))
            .map(Result::unwrap),
    );

    println!("Floor: {floor}");
}

#[cfg(test)]
mod tests {
    use super::get_floor;

    #[test]
    fn test_get_floor() {
        assert_eq!(get_floor("".chars()), 0);
        assert_eq!(get_floor("(())".chars()), 0);
        assert_eq!(get_floor("()()".chars()), 0);
        assert_eq!(get_floor("(((".chars()), 3);
        assert_eq!(get_floor("(()(()(".chars()), 3);
        assert_eq!(get_floor("))(((((".chars()), 3);
        assert_eq!(get_floor("())".chars()), -1);
        assert_eq!(get_floor("))(".chars()), -1);
        assert_eq!(get_floor(")))".chars()), -3);
        assert_eq!(get_floor(")())())".chars()), -3);
    }
}
