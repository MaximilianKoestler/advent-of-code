//! Advent of code 2015 day 12 part 1

use std::io::Read;

fn extract_numbers(chars: impl Iterator<Item = char>) -> impl Iterator<Item = i64> {
    let mut number = 0;
    let mut is_negative = false;
    let mut is_number = false;
    chars.filter_map(move |c: char| {
        if c == '-' {
            is_negative = true;
            None
        } else if c.is_ascii_digit() {
            is_number = true;
            number = number * 10 + i64::from(c.to_digit(10).unwrap());
            None
        } else if is_number {
            let result = if is_negative { -number } else { number };
            number = 0;
            is_negative = false;
            is_number = false;
            Some(result)
        } else {
            is_negative = false;
            None
        }
    })
}

fn main() {
    let file = std::fs::File::open("input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let numbers = extract_numbers(
        reader
            .bytes()
            .map(|b| b.and_then(common::ascii_byte_to_char))
            .map(Result::unwrap),
    );
    let result = numbers.sum::<i64>();

    println!("Sum of all numbers: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array() {
        let numbers: Vec<_> = extract_numbers("[1,2,3]".chars()).collect();
        assert_eq!(numbers, vec![1, 2, 3]);
    }

    #[test]
    fn test_object() {
        let numbers: Vec<_> = extract_numbers(r#"{"a":2,"b":4}"#.chars()).collect();
        assert_eq!(numbers, vec![2, 4]);
    }

    #[test]
    fn test_nested_list() {
        let numbers: Vec<_> = extract_numbers("[[[3]]]".chars()).collect();
        assert_eq!(numbers, vec![3]);
    }

    #[test]
    fn test_nested_object() {
        let numbers: Vec<_> = extract_numbers(r#"{"a":{"b":4},"c":-1}"#.chars()).collect();
        assert_eq!(numbers, vec![4, -1]);
    }

    #[test]
    fn test_array_in_object() {
        let numbers: Vec<_> = extract_numbers(r#"{"a":[-1,1]}"#.chars()).collect();
        assert_eq!(numbers, vec![-1, 1]);
    }

    #[test]
    fn test_object_in_array() {
        let numbers: Vec<_> = extract_numbers(r#"[-1,{"a":1}]"#.chars()).collect();
        assert_eq!(numbers, vec![-1, 1]);
    }

    #[test]
    fn test_empty_array() {
        let numbers: Vec<_> = extract_numbers(r#"[]"#.chars()).collect();
        assert_eq!(numbers, vec![]);
    }

    #[test]
    fn test_empty_object() {
        let numbers: Vec<_> = extract_numbers(r#"{}"#.chars()).collect();
        assert_eq!(numbers, vec![]);
    }
}
