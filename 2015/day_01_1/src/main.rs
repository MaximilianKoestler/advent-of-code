//! Advent of code 2015 day 1 part 2
//! Restrictions for today:
//!   - As many iterator adaptors as possible
//!   - No manual loops
//!   - No non-std dependencies

use std::io::Read;

/// Gets the final floor after following all instructions.
fn get_floor(chars: impl Iterator<Item = char>) -> i32 {
    chars.fold(0, |floor, c| match c {
        '(' => floor + 1,
        ')' => floor - 1,
        _ => floor,
    })
}

fn main() {
    let file = std::fs::File::open("input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let floor = get_floor(reader.bytes().map(|b| b.unwrap()).map(|b| {
        assert!(b.is_ascii());
        b as char
    }));

    println!("Floor: {}", floor);
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
