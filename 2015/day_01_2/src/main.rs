//! Advent of code 2015 day 1 part 2
//! Restrictions for today:
//!   - As many iterator adaptors as possible
//!   - No manual loops
//!   - No external non-std dependencies

use std::io::Read;

/// Gets the position of the first character that causes the floor to go below 0.
/// Returns None if the floor never goes below 0.
fn get_basement_position(chars: impl Iterator<Item = char>) -> Option<usize> {
    chars
        .enumerate()
        .scan(0, |floor, (position, c)| {
            match c {
                '(' => {
                    *floor += 1;
                }
                ')' => {
                    *floor -= 1;
                }
                _ => (),
            }
            Some((*floor, position + 1)) // 1-indexed
        })
        .scan(false, |basement, (floor, position)| {
            // this is like take_while, but with a look-ahead of one, so we include the last element
            if *basement {
                None
            } else {
                if floor < 0 {
                    *basement = true;
                }
                Some((floor, position))
            }
        })
        .last()
        .and_then(
            |(floor, position)| {
                if floor < 0 {
                    Some(position)
                } else {
                    None
                }
            },
        )
}

fn main() {
    let file = std::fs::File::open("input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let floor = get_basement_position(
        reader
            .bytes()
            .map(|b| b.and_then(common::ascii_byte_to_char))
            .map(Result::unwrap),
    )
    .unwrap();

    println!("Basement position: {}", floor);
}

#[cfg(test)]
mod tests {
    use super::get_basement_position;

    #[test]
    fn test_get_basement_position() {
        assert_eq!(get_basement_position(")".chars()), Some(1));
        assert_eq!(get_basement_position("()())".chars()), Some(5));
        assert_eq!(get_basement_position("()()))".chars()), Some(5));

        assert_eq!(get_basement_position("".chars()), None);
        assert_eq!(get_basement_position("(".chars()), None);
        assert_eq!(get_basement_position("()".chars()), None);
    }
}
