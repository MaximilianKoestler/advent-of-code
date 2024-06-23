//! Advent of code 2015 day 12 part 2

use serde_json::Value;
use std::io::Read;

fn extract_non_red_numbers<'a>(root: &'a Value) -> Box<dyn Iterator<Item = i64> + 'a> {
    if let Value::Array(array) = root {
        Box::new(array.iter().flat_map(extract_non_red_numbers))
    } else if let Value::Object(object) = root {
        if object.values().any(|v| v == "red") {
            Box::new(std::iter::empty::<i64>())
        } else {
            Box::new(object.values().flat_map(extract_non_red_numbers))
        }
    } else if let Value::Number(number) = root {
        Box::new(std::iter::once(number.as_i64().unwrap()))
    } else {
        Box::new(std::iter::empty::<i64>())
    }
}

fn main() {
    let file = std::fs::File::open("../day_12_1/input/input.txt").unwrap();
    let mut reader = std::io::BufReader::new(file);

    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).unwrap();
    let root: Value = serde_json::from_str(&buffer).unwrap();

    let numbers = extract_non_red_numbers(&root);
    let result = numbers.sum::<i64>();

    println!("Sum of all numbers: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array() {
        let numbers: Vec<_> =
            extract_non_red_numbers(&serde_json::from_str("[1,2,3]").unwrap()).collect();
        assert_eq!(numbers, vec![1, 2, 3]);
    }

    #[test]
    fn test_object_with_red() {
        let numbers: Vec<_> =
            extract_non_red_numbers(&serde_json::from_str(r#"[1,{"c":"red","b":2},3] "#).unwrap())
                .collect();
        assert_eq!(numbers, vec![1, 3]);
    }

    #[test]
    fn test_object_with_top_level_red() {
        let numbers: Vec<_> = extract_non_red_numbers(
            &serde_json::from_str(r#"{"d":"red","e":[1,2,3,4],"f":5}"#).unwrap(),
        )
        .collect();
        assert_eq!(numbers, Vec::<i64>::new());
    }

    #[test]
    fn test_array_with_red() {
        let numbers: Vec<_> =
            extract_non_red_numbers(&serde_json::from_str(r#"[1,"red",5]"#).unwrap()).collect();
        assert_eq!(numbers, vec![1, 5]);
    }
}
