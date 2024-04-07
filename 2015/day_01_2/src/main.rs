use std::io::Read;

fn get_basement_position(chars: impl Iterator<Item = char>) -> usize {
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
            Some((*floor, position + 1))
        })
        .take_while(|(floor, _)| *floor >= 0)
        .map(|(_, position)| position + 1)
        .last()
        .unwrap_or(1)
}

fn main() {
    let file = std::fs::File::open("input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let floor = get_basement_position(reader.bytes().map(|b| b.unwrap()).map(|b| {
        assert!(b.is_ascii());
        b as char
    }));

    println!("Basement position: {}", floor);
}

#[cfg(test)]
mod tests {
    use super::get_basement_position;

    #[test]
    fn test_get_basement_position() {
        assert_eq!(get_basement_position(")".chars()), 1);
        assert_eq!(get_basement_position("()())".chars()), 5);
    }
}
